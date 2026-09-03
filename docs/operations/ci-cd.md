# CI/CD Workflows & Release Automation

ZenLavaTerm uses GitHub Actions for continuous integration, multi-platform packaging validation, and automated release publishing.

---

## 1. Workflow Architecture Overview

```text
[ Git Push / Pull Request ]
            │
            ▼
┌──────────────────────────────────────────────┐
│  ci.yml (Continuous Integration)            │
│  - lint-and-format (cargo fmt, clippy, check)│
│  - test-linux (tests, smoke test, benchmark) │
│  - test-cross-platform (macOS, Windows)      │
│  - security-audit (RustSec audit-check)      │
└──────────────────────────────────────────────┘

[ Git Tag Push: v*-rc*, v*-beta* ]
            │
            ▼
┌──────────────────────────────────────────────┐
│  package.yml (Packaging Validation)          │
│  - Builds AppImage, DEB, MSI, DMG            │
│  - Verifies package integrity & checksums    │
│  - Builds SHA256SUMS.txt manifest            │
└──────────────────────────────────────────────┘

[ Git Tag Push: v* (Strict SemVer: vX.Y.Z) ]
            │
            ▼
┌──────────────────────────────────────────────┐
│  release.yml (Production Release Automation) │
│  - Validates tag matches Cargo.toml version  │
│  - Builds & tests all 4 desktop installers   │
│  - Generates consolidated SHA256SUMS.txt     │
│  - Attests build provenance (SLSA)          │
│  - Updates Homebrew & AUR manifests          │
│  - Creates GitHub Release with assets        │
└──────────────────────────────────────────────┘
```

---

## 2. Workflows Specification

### 2.1 `.github/workflows/ci.yml`
Runs on every push to `main` or `dev`, and all pull requests:
- **`lint-and-format`**: Checks `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo check`.
- **`test-linux`**: Runs all 135 unit & integration tests on Ubuntu x86_64, compiles benchmarks, runs headless CLI smoke tests, builds release binary, and executes `scripts/smoke_test.py`.
- **`test-cross-platform`**: Matrix test on macOS Apple Silicon (`aarch64-apple-darwin`) and Windows MSVC (`x86_64-pc-windows-msvc`).
- **`cross-target-check`**: Fast verification of macOS Intel (`x86_64-apple-darwin`).
- **`security-audit`**: Runs RustSec security vulnerability audit via `rustsec/audit-check`.

### 2.2 `.github/workflows/package.yml`
Triggered manually (`workflow_dispatch`) or on pre-release tags (`v*-rc*`, `v*-beta*`, `v*-alpha*`):
- Builds packages on Linux, Windows, and macOS without publishing a public GitHub release.
- Validates that packages install or unpack correctly and uploads staging artifacts with retention of 3 days.

### 2.3 `.github/workflows/release.yml`
Triggered on production tags matching `v*`:
- **Strict SemVer Tag Verification**: Enforces regex `^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$`. Rejects pre-release candidates.
- **Cargo Version Consistency**: Fails if the git tag does not match `[package] version` in `Cargo.toml`.
- **Packaging Jobs**: Concurrently packages Linux AppImage/DEB, Windows MSI, and macOS Universal DMG.
- **Publishing Job**: Downloads artifacts, validates builder checksums, writes `SHA256SUMS.txt`, signs SLSA provenance attestations, updates Homebrew/AUR package manifests, and publishes the official GitHub Release.
