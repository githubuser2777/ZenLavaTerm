# ZenLavaTerm Agent Operating Guidelines

`AGENTS.md` is the primary repository-wide instruction entry point for AI coding agents (Antigravity, Claude Code, Cursor, Codex, and other LLM assistants). All agents working in this repository must adhere to the policies, boundaries, and validation workflows defined here.

---

## 1. Project Overview & Reality Check

- **Project**: ZenLavaTerm (crate and binary name: `lavaterm`)
- **Language & Edition**: Rust (Edition 2021, `rustc 1.85+`)
- **Current Version**: `1.0.1` (see `Cargo.toml` and `CHANGELOG.md`)
- **Domain**: Terminal-native ambient lava lamp and metaball visualizer.
- **Frontend / UI Paradigm**: **Terminal-native ANSI TUI** powered by `crossterm`, virtual framebuffer rasterization, and sub-cell Unicode character packing (`▀` half-blocks, `█` full-blocks, `U+2800`..`U+28FF` Braille). **This is NOT a webview or GUI application (no Tauri, no Electron, no WebGL).**

---

## 2. Documentation & Authority Hierarchy

When resolving conflicting information, follow this authoritative precedence order:

1. **Working Implementation & Tests** (`src/`, `tests/`, `benches/`, `Cargo.toml`):
   - The compiled code and test suite define actual runtime behavior and ground truth.
2. **Authoritative Human Documentation** (`docs/`, `README.md`, `CHANGELOG.md`):
   - Source of truth for specifications, architecture, public APIs, CLI flags, configuration schemas, and release notes.
3. **Repository Agent Instructions** (`AGENTS.md`, `.cursor/rules/*.mdc`):
   - Source of truth for agent execution rules, architectural constraints, and coding standards.
4. **AI Supporting Context Cache** (`.ai/context/`, `.ai/tasks/`, `.ai/decisions/`, `.ai/prompts/`):
   - Subordinate AI scratchpad and context cache. If `.ai/` files drift from `docs/` or code, update `.ai/` to match `docs/` and code—never the reverse.

---

## 3. Mandatory Pre-Change Protocol

Before modifying or creating any code, configuration, or documentation:

1. **Inspect Existing Files**:
   - Read the target module, its public exports (`mod.rs`, `lib.rs`), and its existing unit/integration tests.
   - Do not assume missing functionality or duplicate existing modules.
2. **Review Relevant Decisions & Architecture**:
   - Check [docs/architecture/](file:///home/skids/Documents/code/ZenLavaTerm/docs/architecture/) and [.ai/decisions/](file:///home/skids/Documents/code/ZenLavaTerm/.ai/decisions/) before proposing changes to data flow or dependencies.
3. **Check Current Clean State**:
   - Verify that working tree is clean with `git status`.
   - Run `cargo check --all-targets --all-features` to ensure baseline integrity.

---

## 4. Architectural Boundaries & Invariants

All changes must strictly preserve these architectural guarantees:

1. **Strict Unidirectional Data Flow**:
   ```text
   Platform / Inputs  ──>  Signals & Interactions
                                 │
                                 ▼
                          Simulation Core (Blobs, Field, Physics)
                                 │
                                 ▼
                          Virtual Framebuffer (RGB grid)
                                 │
                                 ▼
                          Terminal Renderer (HalfBlock, Block, Braille)
                                 │
                                 ▼
                          Stdout TTY Stream (Batched ANSI bytes)
   ```
2. **`core` is Pure and Zero-Dependency**:
   - `src/core/` operates exclusively in normalized continuous space (`[0.0, 1.0]`).
   - `src/core/` **MUST NOT** import `crossterm`, OS telemetry, audio APIs, or terminal rendering code.
3. **Normalized Telemetry Signals**:
   - `src/reactive/` and `src/audio/` translate hardware and OS telemetry into normalized floats bounded strictly in `[0.0, 1.0]`.
   - Hardware capture backends must fail gracefully to deterministic synthetics (`MockSystemProvider`, `SyntheticAudioGenerator`) so the simulation never freezes or crashes when hardware is unavailable.
4. **Lock-Free Concurrency for Audio**:
   - `src/audio/ring_buffer.rs` (`PcmRingBuffer`) operates on an SPSC model with a 64-bit Seqlock (`version: AtomicU64`) and an atomic CAS producer guard. Readers never block writers and must guarantee tear-free snapshots.
5. **No Production Panics**:
   - All runtime paths must return `Result<T, LavaError>`.
   - Never use `unwrap()`, `expect()`, or `panic!()` in production paths. Error paths must degrade gracefully or return descriptive errors.

---

## 5. Mandatory Post-Change Validation Checklist

Every agent modifying code must execute and verify the following commands before completing a task:

```bash
# 1. Check code formatting
cargo fmt --check

# 2. Run static analysis with zero warnings permitted
cargo clippy --all-targets --all-features -- -D warnings

# 3. Run full automated test suite (unit + integration tests)
cargo test

# 4. Run headless smoke test (runtime lifecycle verification)
cargo run -- --headless --frames 30
```

If benchmarks, packaging, or release scripts are touched, also run:
```bash
# Build benchmarks without regression
cargo bench --no-run

# Run full PTY smoke test suite
python3 scripts/smoke_test.py target/debug/lavaterm
```

---

## 6. Documentation Synchronization Policy

- Whenever modifying CLI arguments, configuration schemas, theme formats, or runtime behaviors:
  1. Update the authoritative guide in [docs/](file:///home/skids/Documents/code/ZenLavaTerm/docs/).
  2. Update [README.md](file:///home/skids/Documents/code/ZenLavaTerm/README.md) if user-facing behavior, install instructions, or CLI flags changed.
  3. Record notable additions, changes, or deprecations in [CHANGELOG.md](file:///home/skids/Documents/code/ZenLavaTerm/CHANGELOG.md) following Keep a Changelog.
  4. Update [.ai/context/current-state.md](file:///home/skids/Documents/code/ZenLavaTerm/.ai/context/current-state.md) if version or test metrics changed.
- Do not invent speculative roadmap features, fake benchmarks, or unverified test counts. Record empirical facts only.

---

## 7. Security & Hygiene Boundaries

- **Zero Secrets / Credentials**: Never place credentials, API keys, tokens, or personal paths in code, configs, or docs.
- **No Generated Artifacts in Git**: Never commit `/target/`, `/dist/`, benchmark logs, raw profile outputs, or temporary agent memory files.
- **Memory Safety & Bounded Resource Usage**: Ensure all allocations in audio buffers, framebuffer grids, and terminal formatting buffers are bounded and validate dimensions to prevent DoS via terminal resize.
