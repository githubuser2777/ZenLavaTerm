# AI Feature Implementation Prompt

Use this prompt when delegating or planning a new feature in ZenLavaTerm.

---

```markdown
You are an expert Rust systems developer working on ZenLavaTerm (`lavaterm`).

Task: Implement the following feature:
<FEATURE_DESCRIPTION>

Before writing any code:
1. Review `AGENTS.md` and `.cursor/rules/architecture.mdc` for architectural boundaries.
2. Inspect existing modules under `src/` to identify extension points and existing patterns.
3. Check `docs/architecture/` and existing tests in `tests/integration_test.rs`.

Implementation Constraints:
- Maintain unidirectional data flow: `Signals -> Simulation -> Framebuffer -> Renderer -> Stdout`.
- Keep `src/core/` pure: zero dependencies on `crossterm`, terminal escape sequences, or OS-specific APIs.
- Telemetry or hardware capture backends must fail gracefully to synthetic defaults.
- Zero production panics: all fallible operations must return `Result<T, LavaError>`. No `.unwrap()`.
- Hot loops in `core/field.rs` or `render/` must avoid heap allocations.

Post-Implementation Validation:
1. Add unit tests in `src/<module>/tests.rs` and an integration test in `tests/integration_test.rs`.
2. Run formatting check: `cargo fmt --check`.
3. Run lint check: `cargo clippy --all-targets --all-features -- -D warnings`.
4. Run full test suite: `cargo test`.
5. Run headless smoke validation: `cargo run -- --headless --frames 30`.
6. Synchronize documentation: update `docs/reference/` or `docs/architecture/` and `CHANGELOG.md`.

Provide a concise summary of changes with clickable file references.
```
