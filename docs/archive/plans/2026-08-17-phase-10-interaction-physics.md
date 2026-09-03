# Phase 10: Interactive Physics & Input Mode Implementation Plan

> **Goal:** Allow users to directly interact with and modulate the fluid metaball lava in real-time via mouse clicks (impact shockwaves), mouse dragging (fluid stirring), mouse scrolling (thermal/buoyancy pressure), and keyboard keypresses (global wave ripples), while maintaining strict architectural decoupling and fail-safe terminal state cleanup.

---

## 1. Architectural Principles & Constraints

LavaTerm adheres to a strict unidirectional layered pipeline:

```text
┌─────────────────────────────────────────────────────────────┐
│                 CLI & Configuration Layer                   │
│   CLI: --no-mouse, --shockwave-force, --stir-force          │
│   TOML: [interaction] section (mouse, keyboard_ripple, etc.)│
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   Input Translation Layer                   │
│   src/input/mouse.rs (crossterm MouseEvent -> Interaction)  │
│   src/input/keyboard.rs (crossterm KeyEvent -> Action)      │
│   src/input/coords.rs (terminal col/row -> sim [0.0, 1.0])  │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Simulation API (Core)                    │
│   src/core/interaction.rs (Interaction domain & kinetics)   │
│   src/core/simulation.rs (apply_interaction, step)          │
│   * 100% unaware of crossterm or terminal dimensions *      │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                  Virtual Framebuffer                        │
│   src/render/framebuffer.rs (VirtualFramebuffer, RGB)       │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Terminal Renderers & Output                 │
│   src/render/ (HalfBlock, Block, Braille)                   │
│   Terminal (Raw mode, alternate screen, mouse capture)      │
└─────────────────────────────────────────────────────────────┘
```

### Invariant Rules
1. **Simulation Decoupling:** `src/core/` MUST NOT import `crossterm` or reference terminal columns/rows. All interaction math operates strictly in continuous normalized space $[0.0, 1.0] \times [0.0, 1.0]$.
2. **Deterministic & Bounded Physics:** All applied impulses must be bounded and damped to prevent velocity explosion or NaN values.
3. **Fail-Safe Terminal Restoration:** Whenever mouse capture is enabled (`crossterm::event::EnableMouseCapture`), it must be reliably disabled (`DisableMouseCapture`) on normal exit, signal interruption (`SIGINT`/`SIGTERM`), handled errors, and inside the panic hook.
4. **Zero Performance Overhead When Inactive:** When mouse capture is disabled or no events are received, the render loop incurs zero additional per-frame allocation.

---

## 2. Phase 10 Structure & Feature Breakdown

```text
Phase 10
├── 10.1 Mouse click → Shockwave
│   ├── Left-click radial explosive impulse pushing blobs outward
│   ├── Smooth inverse-distance falloff with soft core
│   └── Thermal agitation gain at detonation point
├── 10.2 Mouse drag → Stirring
│   ├── Left-drag motion vector calculation (dx, dy)
│   ├── Momentum transfer within influence radius
│   └── Continuous drag velocity clamping and boundary safety
└── 10.3 Keyboard → Ripple
    ├── Alphanumeric key typing event detection
    ├── Harmonic acoustic ripple wave perturbation
    └── Bounded thermal fluctuation across all metaball particles
```

---

## 3. Issues Breakdown

### Issue 30: Interaction Domain Model & Simulation Physics (Core)
- **Goal:** Implement the `Interaction` domain enum, shockwave radial impulse, fluid stirring momentum transfer, keyboard ripple perturbation, and scroll pressure modulation in `src/core/`.
- **Scope:** `src/core/interaction.rs`, `src/core/simulation.rs`, `src/core/mod.rs`.
- **TDD:**
  - Unit tests verifying radial repulsion from shockwave center with inverse-distance falloff.
  - Unit tests verifying directional velocity transfer during drag stirring.
  - Unit tests verifying global ripple wave perturbation and thermal fluctuation.
  - Unit tests verifying bounded delta and velocity clamping preventing simulation instability.

### Issue 31: Terminal Input Translation & Coordinate Normalizer
- **Goal:** Implement conversion of raw `crossterm::event::MouseEvent` and `KeyEvent` into high-level domain `Interaction` and `Action` types.
- **Scope:** `src/input/mouse.rs`, `src/input/coords.rs`, `src/input/keyboard.rs`, `src/input/mod.rs`.
- **TDD:**
  - Unit tests converting terminal cell coordinates `(col, row)` across arbitrary aspect ratios into continuous normalized $[0.0, 1.0]$ simulation coordinates (accounting for vertical inversion between terminal row 0 and simulation bottom heat plate $y=0$).
  - Unit tests tracking drag motion vectors across successive mouse events.
  - Unit tests mapping mouse scroll up/down to pressure increments.
  - Unit tests for keyboard ripple triggers on character keys.

### Issue 32: Terminal Mouse Capture Lifecycle, Panic Hook & Signal Safety
- **Goal:** Implement safe initialization and teardown of terminal mouse capture mode in `src/main.rs`.
- **Scope:** `src/main.rs`.
- **Safety:**
  - Ensure `EnableMouseCapture` and `DisableMouseCapture` are executed paired.
  - Extend `setup_panic_hook()` and `restore_terminal()` to disable mouse capture.
  - Signal hook for `SIGINT`/`SIGTERM` cleans up mouse capture.

### Issue 33: Configuration `[interaction]`, CLI Switches & Policy Integration
- **Goal:** Add `[interaction]` section to TOML config and CLI switches (`--no-mouse`, `--shockwave-force`, `--stir-force`).
- **Scope:** `src/config/schema.rs`, `src/main.rs`.
- **Validation:**
  - Validate strength multipliers ($> 0.0$).
  - Test TOML parsing and serialization for `[interaction]`.

### Issue 34: Phase 10 Comprehensive Test Suite & Documentation
- **Goal:** Provide end-to-end integration tests and update all architectural and user-facing documentation.
- **Scope:** `tests/integration_test.rs`, `docs/architecture.md`, `docs/configuration.md`, `docs/roadmap.md`, `docs/github_issues.md`, `README.md`, `CHANGELOG.md`.
- **Quality Gates:**
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`
  - `cargo build`
