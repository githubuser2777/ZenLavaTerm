# AI Bug Investigation & Fix Prompt

Use this prompt when diagnosing, reproducing, and resolving bugs in ZenLavaTerm.

---

```markdown
You are an expert Rust systems engineer investigating a defect in ZenLavaTerm (`lavaterm`).

Bug Report:
<BUG_DESCRIPTION_OR_ISSUE_CONTENT>

Investigation Protocol:
1. Inspect the relevant module under `src/` and read the corresponding tests in `tests/integration_test.rs`.
2. Formulate a hypothesis for root cause. Check for:
   - Coordinate inversion or terminal boundary off-by-one errors (`src/input/coords.rs`, `src/render/framebuffer.rs`).
   - Concurrency race conditions or Seqlock snapshot retry limits (`src/audio/ring_buffer.rs`).
   - Operating system API differences or missing telemetry fallbacks (`src/reactive/`).
   - Configuration deserialization or schema validation issues (`src/config/schema.rs`).
3. Construct a minimal failing reproduction test in `tests/integration_test.rs` or unit test module before making the fix.

Fix Constraints:
- Minimal surgical fix: do not rewrite surrounding architecture or refactor unrelated code.
- Zero production panics: ensure all error paths return `Result<T, LavaError>` or degrade gracefully.
- Preserve unidirectional data flow and core zero-dependency invariant.

Verification:
1. Run the new regression test to confirm the bug is resolved.
2. Run `cargo fmt --check`.
3. Run `cargo clippy --all-targets --all-features -- -D warnings`.
4. Run `cargo test` to ensure zero regressions across all existing tests.
5. Run `cargo run -- --headless --frames 30`.
6. Update `CHANGELOG.md` under `### Fixed`.
```
