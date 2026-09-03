# Phase 9: Multiplexer & Widget Mode Implementation Plan (Revised)

> **Goal:** Enable LavaTerm to run seamlessly as an ambient widget, status-bar item, or compact split-pane within modern terminal multiplexers (`tmux`, `zellij`) and lightweight desktop workflows without high CPU usage, startup noise, or visual distortion.

---

## 1. Architectural Principles & Constraints

LavaTerm follows a strict unidirectional layered architecture. Phase 9 introduces the `widget` subsystem as a layer **above** the simulation engine, preserving full modularity:

```text
┌─────────────────────────────────────────────────────────────┐
│                 CLI & Configuration Layer                   │
│   CLI: --fps, --compact, --widget, --inline, --snapshot     │
│   TOML: [widget] section (compact, fps, inline, width/height)│
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   Widget Policy Layer                       │
│  src/widget/policy.rs (ExecutionMode, precedence, validation)│
└──────────────┬───────────────────────────────┬──────────────┘
               │                               │
               ▼                               ▼
┌──────────────────────────────┐ ┌────────────────────────────┐
│    Environment Adapter       │ │       Compact Scaler       │
│  src/widget/multiplexer.rs   │ │  src/widget/compact.rs     │
│  (Pure env detection: tmux/  │ │  (should_compact, profile- │
│   zellij via DI abstraction) │ │   based CompactProfile)    │
└──────────────┬───────────────┘ └─────────────┬──────────────┘
               │                               │
               └───────────────┬───────────────┘
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Simulation API (Core)                    │
│    src/core/ (Simulation, PhysicsParams, step)              │
│    * Unaware of tmux, zellij, widget, inline, snapshot *    │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                  Virtual Framebuffer                        │
│    src/render/framebuffer.rs (VirtualFramebuffer, RGB)      │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Terminal Renderers & Output                 │
│    src/render/ (HalfBlock, Block, Braille)                  │
│    src/widget/snapshot.rs (Single-shot ANSI serializer)     │
│    Terminal (Alternate screen OR safe inline reposition)    │
└─────────────────────────────────────────────────────────────┘
```

### Invariant Rules
1. **Simulation Unawareness:** The simulation core (`src/core/`) must remain 100% unaware of terminal multiplexers, widget mode, inline mode, or snapshot mode.
2. **No Duplicated Logic:** No duplicate simulation engines, no duplicate renderers, and no widget-specific physics forks.
3. **Pure Environment Adapter:** Multiplexer detection only reports environment information; it does not dictate FPS, rendering modes, or compact policies.
4. **Deterministic Testing:** Environment detection, geometry scaling, and snapshot serialization must be testable in isolation via clean data contracts (e.g. `render_snapshot` returning `Result<String, LavaError>` and environment lookup abstraction) without mutating global process state.

---

## 2. Core Concepts & Subsystems

### 2.1. Separation of Compact Activation from Compact Scaling
Compact behavior is split into two distinct, non-conflicting steps:

1. **Activation Policy (`should_compact`):**
   - **Automatic:** Active when terminal geometry is constrained: `cols < 40 || rows < 15`.
   - **Explicit:** Forced when `--compact` or `--widget` CLI switch is passed, or when `[widget].compact = true` in config.
2. **Profile-Based Scaler (`CompactScaler`):**
   - Takes active viewport geometry `(cols, rows)` and base configuration, returning a `CompactProfile`:
     ```rust
     #[derive(Debug, Clone, Copy, PartialEq)]
     pub struct CompactProfile {
         pub blob_count: usize,
         pub radius_scale: f32,
         pub buoyancy_scale: f32,
         pub noise_scale: f32,
     }
     ```
   - Uses viewport area and aspect ratio internally to calculate the profile deterministically based on viewport geometry without redefining whether compact mode is active.
   - The profile is applied directly to simulation initialization and physics parameters before starting execution.

### 2.2. Widget Policy Layer (`src/widget/policy.rs`)
Centralizes execution decisions to keep `src/main.rs` clean:
- **`ExecutionMode` Enum:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum ExecutionMode {
      Interactive, // Full-screen alternate-screen loop
      Widget,      // Low-overhead loop with compact defaults (15 FPS)
      Inline,      // Interactive rendering within current terminal lines
      Snapshot,    // Single-shot ANSI True Color output to stdout
  }
  ```
- **Validation & Conflict Resolution:**
  - Rejects mutually exclusive combinations (e.g. `--snapshot` with `--inline` or `--headless`) with clear, informative errors.
- **Precedence Management:**
  $$\text{CLI Argument} > \text{TOML Configuration} > \text{Built-in Default}$$

### 2.3. Multiplexer Environment Adapter (`src/widget/multiplexer.rs`)
- **`MultiplexerKind`:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum MultiplexerKind {
      Tmux,
      Zellij,
      GenericTerminal,
  }
  ```
- **Detection Contract:**
  - Primary checks: presence of `TMUX` and `ZELLIJ` variables.
  - Supplementary: `TERM_PROGRAM` (read-only diagnostic).
  - Testable via an `EnvLookup` abstraction (e.g. `fn detect_multiplexer_from<E: EnvLookup>(env: &E) -> MultiplexerKind`).

### 2.4. Snapshot Mode Contract (`--snapshot`, `src/widget/snapshot.rs`)
- **Purpose:** Single-shot ANSI True Color string generation for status bars (tmux `status-right`, zellij plugins, polybar, shell scripts).
- **Execution & Serialization Contract:**
  - `render_snapshot(...) -> Result<String, LavaError>` functions as a pure ANSI frame serializer that returns the rendered frame as a self-contained `String`.
  - The execution layer (`src/main.rs` / CLI runner) receives this string and writes it to `stdout`.
  - **No** `EnterAlternateScreen`.
  - **No** raw terminal mode.
  - **No** cursor positioning or clearing escapes.
  - **No** event loop or keyboard polling.
  - **No** terminal teardown/cleanup sequences.
  - Clean process exit code `0`.
- **Supported Renderers & Micro-geometries:** Full support for `halfblock`, `block`, and `braille` across tiny viewports (`20x1`, `20x2`, `20x3`, `24x8`, `80x24`).

### 2.5. Inline Mode Contract (`--inline`)
- **Purpose:** Interactive live visualizer running in-place without taking over the full alternate screen buffer.
- **Execution Contract:**
  - Operates inside the current cursor region using ANSI relative cursor movement.
  - Leaves surrounding terminal content / scrollback intact.
  - Cross-platform safe restoration:
    - Restores terminal raw mode and cursor visibility.
    - Gracefully handles normal exit (`q` / `Esc`), `Ctrl-C` / `SIGINT`, `SIGTERM`, `SIGHUP` (on Unix), and panics (via panic hook).
    - Platform distinction: Uses standard signal handling on Unix and standard termination/Ctrl-C handlers on Windows. Shell prompt is never corrupted on exit.

### 2.6. CLI Flags & Precedence Rules
- **CLI Extensions:**
  - `--fps <FPS>`: Explicit target frame rate override.
  - `--compact`: Force compact geometry & profile scaling (visual policy).
  - `--widget`: Low-overhead execution mode (resource policy: implies compact scaling, defaults to 15 FPS).
  - `--inline`: Render in-place without entering alternate screen.
  - `--snapshot`: Single-shot ANSI string output to stdout and exit.
  - `--width <COLS>` / `--height <ROWS>`: Explicit viewport dimensions override.
- **Precedence & Defaults:**
  - Normal Interactive mode: Default 30 FPS (from config or built-in).
  - Widget mode: Default 15 FPS (overridden by `--fps` or `[widget].fps`).
  - Snapshot mode: Fixed 1 frame.
  - Explicit `--fps <N>` always takes highest priority.

---

## 3. Milestone Issues Specification

### Issue 26: Multiplexer Environment Adapter
- **Goal:** Implement `src/widget/multiplexer.rs` providing pure environment detection without mixing in rendering or policy logic.
- **Scope:** `src/widget/mod.rs`, `src/widget/multiplexer.rs`.
- **Acceptance Criteria:**
  - `MultiplexerKind` enum (`Tmux`, `Zellij`, `GenericTerminal`).
  - Decoupled `detect_multiplexer_with(getter)` allowing deterministic testing without mutating process-wide environment variables.
  - Unit tests covering `TMUX`, `ZELLIJ`, and generic terminal detection.
- **Non-Goals:** Deciding FPS, switching renderers, or altering screen buffers.

### Issue 27: Adaptive Geometry & Compact Profiles
- **Goal:** Implement `src/widget/compact.rs` separating compact activation from profile-based physics parameter scaling.
- **Scope:** `src/widget/compact.rs`.
- **Acceptance Criteria:**
  - `should_compact(cols, rows, explicit_flag)` determines activation cleanly.
  - `CompactScaler::calculate_profile(cols, rows, base_config)` returns `CompactProfile`.
  - Maps blob count, radii, buoyancy, and noise deterministically based on viewport geometry.
  - Unit tests across full range of viewports: `10x3`, `15x5`, `20x8`, `24x8`, `40x15`, `80x24`, `200x60`.
- **Non-Goals:** Multiplexer-specific physics branching or modifying `src/core/`.

### Issue 28: Widget Output Modes (Snapshot & Inline)
- **Goal:** Implement `src/widget/snapshot.rs` and inline frame serialization with strict terminal-safety guarantees.
- **Scope:** `src/widget/snapshot.rs`, `src/render/mod.rs`.
- **Acceptance Criteria:**
  - `render_snapshot(...) -> Result<String, LavaError>` serializes and returns a clean ANSI True Color `String` with zero terminal state changes, allowing the execution layer to output it to `stdout` and enabling direct unit testing.
  - Support for micro-geometries (`20x1`, `20x2`, `20x3`, `24x8`, `80x24`) across `halfblock`, `block`, and `braille`.
  - Inline terminal rendering pipeline operating without `EnterAlternateScreen`.
  - Unit tests directly asserting on serialized ANSI string structure and color sequence correctness.
- **Non-Goals:** Interactive keyboard handling in snapshot mode.

### Issue 29: Policy Layer, CLI Integration, Configuration, Signals & Full Test Suite
- **Goal:** Implement `src/widget/policy.rs`, update `src/config/schema.rs`, integrate CLI in `src/main.rs`, add cross-platform signal hooks, and comprehensive integration tests.
- **Scope:** `src/widget/policy.rs`, `src/config/schema.rs`, `src/main.rs`, `src/lib.rs`, `tests/integration_test.rs`, `docs/`.
- **Acceptance Criteria:**
  - `src/widget/policy.rs` validates CLI/TOML combinations and resolves `ExecutionMode` and final `RuntimeOptions`.
  - CLI flags: `--fps`, `--compact`, `--widget`, `--inline`, `--snapshot`, `--width`, `--height`.
  - TOML `[widget]` schema with `compact`, `fps`, `inline`, `width`, `height`, `adapt_blobs`.
  - Strict conflict validation (e.g. `--snapshot` combined with `--inline` fails gracefully).
  - Cross-platform terminal restoration on normal exit, panic, and signals (`SIGINT`, `SIGTERM`, `SIGHUP` on Unix; `Ctrl-C` on Windows).
  - Comprehensive integration tests covering all execution modes, snapshot outputs, and config overrides.
  - All quality gates pass (`cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo build --release`).

---

## 4. Verification & Testing Strategy

### 4.1. Unit Test Matrix
- **Environment Detection (`src/widget/multiplexer.rs`):**
  - Simulated `TMUX=...` -> `MultiplexerKind::Tmux`
  - Simulated `ZELLIJ=...` -> `MultiplexerKind::Zellij`
  - Empty environment -> `MultiplexerKind::GenericTerminal`
- **Compact Geometry Scaling (`src/widget/compact.rs`):**
  - Micro panes: `10x3`, `15x5`, `20x8` (scaled blob count 2–4, reduced radii)
  - Small panes: `24x8`, `40x15` (scaled blob count 4–8)
  - Standard/Large panes: `80x24`, `200x60` (full blob count, 1.0 radius scale)
- **Snapshot Serializer (`src/widget/snapshot.rs`):**
  - Tiny viewports: `20x1`, `20x2`, `20x3`
  - Standard viewports: `24x8`, `80x24`
  - Renderer variants: `halfblock`, `block`, `braille`
  - Unit tests directly asserting on returned `String` containing valid 24-bit True Color sequences and ending with reset.
- **Policy & Validation (`src/widget/policy.rs`):**
  - Precedence: CLI `--fps 10` overrides `[widget] fps = 20` and default 15.
  - Conflict: `--snapshot` + `--inline` returns validation error.

### 4.2. Integration Test Matrix (`tests/integration_test.rs`)
- End-to-end headless snapshot generation with `--theme cyberpunk` and `--renderer braille`.
- End-to-end compact mode simulation step verification.
- TOML `[widget]` deserialization and precedence resolution.

### 4.3. Quality Gates
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

---

## 5. Scope & Status

> [!NOTE]
> **Status:** Phase 9 remains **PLANNED** (Specification Revised).
> No Rust source files, CI/CD configurations, or package manifests have been altered. Implementation will commence only after explicit approval.
