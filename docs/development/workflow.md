# Development Workflow & Quality Gates

This guide outlines our development lifecycle, git branching conventions, commit standards, and mandatory pre-flight checks.

---

## 1. Branching & Git Workflow

- **`main`**: Production release branch. All code must pass CI and be tagged with SemVer tags (`vX.Y.Z`).
- **Feature / Fix Branches**: Branch from `main` using descriptive names:
  - `feat/<short-description>`
  - `fix/<short-description>`
  - `perf/<short-description>`
  - `refactor/<short-description>`
- Keep pull requests focused on a single change or issue.

---

## 2. Commit Message Conventions

ZenLavaTerm enforces [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<scope>): <concise description in imperative mood>

[optional body explaining context and rationale]

[optional footer with issue reference, e.g. Closes #42]
```

- **Allowed Types**: `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `chore`, `ci`, `release`.
- **Allowed Scopes**: `core`, `render`, `audio`, `reactive`, `input`, `config`, `widget`, `theme`, `packaging`, `ci`.

---

## 3. Mandatory Pre-Flight Validation Gates

Before submitting a PR or pushing to `main`, execute and verify the four standard quality gates:

```bash
# Gate 1: Code Formatting
cargo fmt --check

# Gate 2: Static Analysis (zero warnings allowed)
cargo clippy --all-targets --all-features -- -D warnings

# Gate 3: Automated Test Suite (135+ tests must pass)
cargo test

# Gate 4: Headless Lifecycle Run
cargo run -- --headless --frames 30
```

### Extended Checks for Audio, Packaging, or CLI Changes
```bash
# Gate 5: Benchmark compilation check
cargo bench --no-run

# Gate 6: Comprehensive PTY smoke test suite
python3 scripts/smoke_test.py target/debug/lavaterm
```
