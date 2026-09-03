# AI Release Preparation & Verification Prompt

Use this prompt when preparing, packaging, and verifying a new production release for ZenLavaTerm.

---

```markdown
You are a Release Engineer guiding a new release of ZenLavaTerm (`lavaterm`).

Target Version: v<TARGET_VERSION> (e.g., v1.0.2)

Pre-Release Verification Checklist:
1. Version Consistency:
   - Check that `[package] version` in `Cargo.toml` matches `<TARGET_VERSION>`.
   - Run `cargo check` to update `Cargo.lock`.
   - Verify that git tag will be `v<TARGET_VERSION>` matching `^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$`.

2. Local Validation Gate:
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test` (verify all unit and integration tests pass)
   - `cargo run -- --headless --frames 30`
   - `python3 scripts/smoke_test.py target/debug/lavaterm`

3. Documentation & Changelog:
   - Update `CHANGELOG.md` with release date, categorized additions, optimizations, and fixes following Keep a Changelog.
   - Update `.ai/context/current-state.md` with new version and verified test metrics.
   - Update `README.md` if installation commands or defaults changed.

4. Packaging Manifests:
   - Run `./scripts/update_package_manifests.sh <TARGET_VERSION>` to test manifest generation for Homebrew and AUR.
   - Verify packaging scripts: `scripts/package_linux.sh`, `scripts/package_macos.sh`, `scripts/package_windows.ps1`.

5. GitHub Workflow Assurance:
   - Verify that `.github/workflows/release.yml` will trigger on push of tag `v<TARGET_VERSION>`.
   - Ensure pre-release flags (`-rc*`, `-beta*`) are NOT present on the production tag.
```
