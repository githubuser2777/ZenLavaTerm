# Contributing to LavaTerm

Thank you for your interest in contributing to **LavaTerm**!

LavaTerm is built with a production-minded, clean-architecture approach in Rust. We value correctness, clear domain boundaries, beautiful terminal aesthetics, and maintainable code over quick hacks.

---

## Development Setup

### Requirements

- **Rust 1.75+** (`rustup default stable`)
- `rustfmt` and `clippy` components (`rustup component add rustfmt clippy`)
- A modern True-Color terminal emulator (Kitty, Alacritty, WezTerm, Ghostty, etc.)

### Build and Test

```bash
# Check code formatting
cargo fmt --check

# Run linter with strict warning checks
cargo clippy --all-targets --all-features -- -D warnings

# Run all unit and integration tests
cargo test

# Run the project in development mode
cargo run

# Run headless simulation test
cargo run -- --headless --frames 30
```

---

## Contribution Workflow

1. **Find or Open an Issue**: Check existing issues or open a new one following the issue templates and the full checklist in `docs/issue_creation_checklist.md`. Every feature or bug fix should correspond to an issue with clear acceptance criteria.
2. **Branch Naming**:
   - `feat/<feature-name>`
   - `fix/<bug-name>`
   - `refactor/<scope>`
   - `docs/<topic>`
3. **Commit Messages**: Follow Conventional Commits format:
   ```text
   feat(core): implement scalar field evaluation
   fix(render): correct halfblock vertical cell index
   test(physics): add buoyancy damping test case
   docs: update simulation math formulas
   ```
4. **Code Quality Standards**:
   - Do not import terminal/TTY dependencies (e.g. `crossterm`) into the `core` simulation module.
   - Do not use `unwrap()` in production paths; use typed errors (`Result<T, LavaError>`).
   - Write unit tests for new physics, math, color, or configuration logic.
   - Run `cargo fmt` and `cargo clippy` before submitting.
5. **Open a Pull Request**: Fill out the PR template completely and ensure all CI checks pass.

---

## Architecture Boundaries

Always respect the unidirectional data flow:
```text
Signals / Inputs -> Simulation Core -> Virtual Framebuffer -> Renderer -> Terminal Backend
```

- **Core**: Pure math and physics. Deterministic, zero terminal I/O.
- **Render**: Virtual canvas and color transformation to ANSI/Unicode primitives.
- **Config**: Parses TOML configurations into validated configuration structs.
- **Terminal Backend**: Handles raw mode, alternate screen, and flushes batched ANSI buffers to stdout.

Thank you for helping make LavaTerm amazing!
