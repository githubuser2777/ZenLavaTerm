# ZenLavaTerm Release Pipeline Context

> **Note**: The authoritative human-facing release playbook is in [docs/releases/process.md](file:///home/skids/Documents/code/ZenLavaTerm/docs/releases/process.md).

---

## 1. Release Identification

- **Current Version**: `1.0.1`
- **SemVer Tag Scheme**: `^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$` (e.g. `v1.0.1`).
- **Tag Verification Rule**: `.github/workflows/release.yml` strictly enforces that the git tag matches `Cargo.toml` package version. Pre-release tags (`-rc`, `-beta`, `-alpha`) are rejected by the production release workflow and must use `.github/workflows/package.yml`.

---

## 2. Release Artifact Matrix

The release workflow produces 4 desktop installers and 2 community package manager bundles:

| Platform | Output Artifact | Build Tool / Script |
|---|---|---|
| **Linux (AppImage)** | `ZenLavaTerm-v<VERSION>-linux-x86_64.AppImage` | `scripts/package_linux.sh` |
| **Linux (Debian/Ubuntu)** | `ZenLavaTerm-v<VERSION>-linux-x86_64.deb` | `scripts/package_linux.sh` (`dpkg-deb`) |
| **Windows (MSI)** | `ZenLavaTerm-v<VERSION>-windows-x86_64.msi` | `scripts/package_windows.ps1` (WiX 3.11/3.14) |
| **macOS (Universal DMG)** | `ZenLavaTerm-v<VERSION>-macos-universal.dmg` | `scripts/package_macos.sh` (`lipo` + `hdiutil`) |
| **Homebrew Formula** | `dist/packaging/homebrew/lavaterm.rb` | `scripts/update_package_manifests.sh` |
| **Arch Linux AUR** | `dist/packaging/aur/PKGBUILD`, `.SRCINFO` | `scripts/update_package_manifests.sh` |

---

## 3. Automated Integrity & Attestation Verification

1. **Independent Checksum Generation**: Each OS builder computes its artifact's SHA256 sum into a `.sha256` sidecar file.
2. **Consolidation**: The release workflow downloads all artifacts, re-verifies them against the builders' `.sha256` sidecars, and generates a unified `SHA256SUMS.txt`.
3. **SLSA Provenance Attestation**: Attestation build provenance (`actions/attest-build-provenance`) is generated for all binary installers.
4. **Manifest Synchronization**: Computes immutable source archive SHA256 (`https://github.com/githubuser2777/ZenLavaTerm/archive/refs/tags/v<VERSION>.tar.gz`) and injects it into Homebrew and AUR manifests.
