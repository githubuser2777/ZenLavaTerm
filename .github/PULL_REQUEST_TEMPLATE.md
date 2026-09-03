## Summary of Changes

A concise description of the changes introduced in this PR.

## Related Issue

Closes # (issue number)

## Architecture Compliance

- [ ] Complies with guidelines in [AGENTS.md](AGENTS.md) and [.cursor/rules/](.cursor/rules/)
- [ ] Unidirectional data flow preserved (`Signals -> Simulation -> Framebuffer -> Renderer -> Terminal`)
- [ ] No terminal or platform dependencies (`crossterm`, `libc`, `windows-sys`) leaked into `core`
- [ ] No `unwrap()`, `expect()`, or `panic!()` in production paths (proper error handling with `Result<T, LavaError>`)
- [ ] No unnecessary external dependencies added to `Cargo.toml`
- [ ] No secrets, credentials, benchmark logs, or temporary agent memory committed

## Testing & Quality Checklist

- [ ] Unit tests added / updated in `src/<module>/tests.rs`
- [ ] Integration tests added / updated in `tests/integration_test.rs`
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes (all 135+ tests pass)
- [ ] `cargo run -- --headless --frames 30` passes
- [ ] Documentation updated in `docs/`, `README.md`, or `CHANGELOG.md` if applicable

## Verification & Notes

Describe or attach notes/logs demonstrating the change in action (e.g. headless run output or benchmark verification).
