## Summary of Changes

A concise description of the changes introduced in this PR.

## Related Issue

Closes # (issue number)

## Architecture Compliance

- [ ] Unidirectional data flow preserved (`Signals -> Simulation -> Framebuffer -> Renderer -> Terminal`)
- [ ] No terminal dependencies (`crossterm`) leaked into `core`
- [ ] No `unwrap()` in production paths (proper error handling with `Result`)
- [ ] No unnecessary external dependencies added

## Testing & Quality Checklist

- [ ] Unit tests added / updated
- [ ] Integration tests pass
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] Documentation updated in `docs/` or `README.md` if applicable

## Visual / Verification Notes

Describe or attach notes/logs demonstrating the change in action.
