# Contributing to LavaTerm

This document serves as the in-depth technical contributor guide for the **LavaTerm** project.

---

## 1. Core Principles

1. **Clean Architectural Boundaries**: The simulation core (`src/core/`) must never import terminal libraries (`crossterm`) or platform-specific audio/system APIs.
2. **Visual Quality First**: Every rendering change must maintain crisp sub-cell resolution and zero flicker.
3. **No Unsafe Code**: The codebase adheres to 100% safe Rust unless explicitly justified and documented.
4. **No `unwrap()` in Production Paths**: Always bubble up errors using `Result<T, LavaError>`.
5. **Deterministic Testing**: Physics and field calculations must have deterministic unit tests using seeded PRNG inputs.

---

## 2. Setting Up Your Environment

### 2.1. Toolchain
Ensure you have the latest stable Rust toolchain:
```bash
rustup update stable
rustup component add clippy rustfmt
```

### 2.2. Running Tests and Checks
Before submitting any pull request, run the standard quality verification suite:

```bash
# 1. Format check
cargo fmt --check

# 2. Clippy lint check
cargo clippy --all-targets --all-features -- -D warnings

# 3. Test execution
cargo test --all-targets

# 4. Release build check
cargo build --release
```

---

## 3. Pull Request Guidelines

- PRs should address a single well-defined GitHub Issue.
- Keep PRs focused; avoid mixing refactorings with new features.
- Update documentation in `docs/` whenever configuration schema, CLI options, or core APIs change.
- Follow the [Conventional Commits](https://www.conventionalcommits.org/) format for all git commits.
