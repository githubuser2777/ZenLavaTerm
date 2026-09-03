# Contributing to ZenLavaTerm

Thank you for your interest in contributing to ZenLavaTerm! We welcome bug fixes, performance optimizations, documentation improvements, and platform support enhancements.

---

## 1. Code of Conduct

All contributors and maintainers are expected to uphold our [Code of Conduct](file:///home/skids/Documents/code/ZenLavaTerm/CODE_OF_CONDUCT.md).

---

## 2. How to Contribute

### 2.1 Reporting Bugs
- Search existing issues before creating a new report.
- Use our [Bug Report Template](file:///home/skids/Documents/code/ZenLavaTerm/.github/ISSUE_TEMPLATE/bug_report.md).
- Include terminal emulator name, operating system, exact command line, and reproduction steps.

### 2.2 Proposing Enhancements
- Open a feature proposal or discuss ideas in GitHub Discussions.
- Respect our core philosophy: ZenLavaTerm is a lightweight, terminal-native visualizer. We avoid heavy web GUI wrappers, bloated dependencies, and features that compromise startup latency or CPU efficiency.

### 2.3 Submitting Pull Requests
1. Fork and create a branch from `main`.
2. Follow the architectural rules outlined in [AGENTS.md](file:///home/skids/Documents/code/ZenLavaTerm/AGENTS.md) and [docs/architecture/](file:///home/skids/Documents/code/ZenLavaTerm/docs/architecture/).
3. Ensure zero production panics: use `Result<T, LavaError>` without `.unwrap()`.
4. Include tests covering new functionality.
5. Run all pre-flight checks:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   cargo run -- --headless --frames 30
   ```
6. Fill out the [Pull Request Template](file:///home/skids/Documents/code/ZenLavaTerm/.github/PULL_REQUEST_TEMPLATE.md).
