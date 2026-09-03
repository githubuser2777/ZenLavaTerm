# Release Engineering & Deployment Playbook

This document describes the automated release workflow and mandatory release engineering steps for ZenLavaTerm.

---

## 1. Release Workflow Overview

ZenLavaTerm releases are automated via GitHub Actions in [.github/workflows/release.yml](file:///home/skids/Documents/code/ZenLavaTerm/.github/workflows/release.yml). Pushing a valid production git tag (`vX.Y.Z`) triggers cross-platform packaging, checksum generation, SLSA provenance generation, and GitHub Release publication.

---

## 2. Step-by-Step Release Checklist

### Step 1: Version Bump & Synchronization
1. Update `[package] version = "X.Y.Z"` in `Cargo.toml`.
2. Update `Cargo.lock`:
   ```bash
   cargo check
   ```
3. Update `CHANGELOG.md` with:
   - Version number and release date: `## [X.Y.Z] - YYYY-MM-DD`
   - Detailed subsections: `### Added`, `### Changed`, `### Deprecated`, `### Removed`, `### Fixed`, `### Security`.

### Step 2: Local Pre-Release Validation
Execute the complete quality validation matrix:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --headless --frames 30
python3 scripts/smoke_test.py target/debug/lavaterm
```

### Step 3: Git Commit & Tagging
Commit changes and create an annotated git tag matching the exact SemVer tag pattern:
```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "release(vX.Y.Z): prepare vX.Y.Z release"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
```

> **Warning**: Production tags MUST strictly match `^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$`. Tags containing suffixes like `-rc*` or `-beta*` will be rejected by `release.yml` and must use `package.yml` instead.

### Step 4: Push Tag & CI Verification
```bash
git push origin main
git push origin vX.Y.Z
```
Monitor the GitHub Actions run under the `Release` workflow:
1. `verify-tag`: Validates tag SemVer syntax and asserts it matches `Cargo.toml`.
2. `package-linux`: Builds AppImage and DEB, validating with `dpkg-deb`.
3. `package-windows`: Builds Windows MSI installer via WiX Toolset.
4. `package-macos`: Builds Universal macOS DMG via `lipo` and `hdiutil`.
5. `publish-release`: Consolidates `SHA256SUMS.txt`, creates SLSA build attestations, updates Homebrew & AUR manifests, and publishes the release.
