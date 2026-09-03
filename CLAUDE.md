# Claude Code Guidance for ZenLavaTerm

This file provides Claude Code specific operational shortcuts and invocation guidelines.
**All repository-wide architectural rules, quality standards, and validation workflows are defined authoritatively in [AGENTS.md](file:///home/skids/Documents/code/ZenLavaTerm/AGENTS.md).** Do not duplicate rules here.

---

## 1. Quick Commands for Claude

```bash
# Fast sanity typecheck
cargo check --all-targets --all-features

# Run formatting check
cargo fmt --check

# Strict lint check (must have 0 warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Run all 135 tests (unit + integration)
cargo test

# Run a single unit test (e.g., ring buffer coherence)
cargo test test_ring_buffer_lock_free_concurrent_producer_consumer

# Run a single integration test
cargo test --test integration_test test_phase12_audio_pipeline_full_e2e_samples_to_render

# Run headless 30-frame validation
cargo run -- --headless --frames 30

# Fast release build
cargo build --release
```

---

## 2. Operating Principles for Claude

1. **Reference AGENTS.md First**:
   - Consult [AGENTS.md](file:///home/skids/Documents/code/ZenLavaTerm/AGENTS.md) for architectural boundaries (`Signals -> Simulation -> Framebuffer -> Renderer -> Stdout`), zero production panics, and documentation requirements.
2. **Inspect Before Changing**:
   - Always view the relevant module files and test files before creating or editing code.
3. **Keep Explanations Concise**:
   - Claude responses should be dense and actionable. Point directly to modified files and symbols.
4. **Always Execute Validation**:
   - Run `cargo fmt --check`, `cargo clippy`, and `cargo test` after any code edits before declaring a task finished.
5. **Update Supporting Docs**:
   - If public behaviors, CLI flags, or schemas change, update the relevant files in `docs/` and synchronize `.ai/context/current-state.md`.
