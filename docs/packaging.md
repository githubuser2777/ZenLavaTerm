# ZenLavaTerm Packaging & Installation Guide

> **Packaging Documentation Hub**: For operational details on building packages and CI/CD automation, see [docs/operations/packaging.md](operations/packaging.md) and [docs/operations/ci-cd.md](operations/ci-cd.md).

This document outlines the official installation methods, supported package formats, and the maintainer release process for **ZenLavaTerm**.

---

## 1. Official Desktop Release Matrix

ZenLavaTerm provides official desktop installers for all tier-1 supported operating systems:

| Operating System | Package Format | Architecture | Canonical Artifact Name | Primary Distribution |
|---|---|---|---|---|
| **Linux** | `.AppImage` | `x86_64` | `ZenLavaTerm-v<VERSION>-linux-x86_64.AppImage` | Standalone portable executable |
| **Linux** | `.deb` | `x86_64` (amd64) | `ZenLavaTerm-v<VERSION>-linux-x86_64.deb` | Debian / Ubuntu package manager |
| **Windows** | `.msi` | `x86_64` | `ZenLavaTerm-v<VERSION>-windows-x86_64.msi` | Windows Installer with PATH registration |
| **macOS** | `.dmg` | Universal (`arm64` + `x86_64`) | `ZenLavaTerm-v<VERSION>-macos-universal.dmg` | Apple Silicon & Intel Universal Bundle |

Every official release includes a consolidated `SHA256SUMS.txt` and SLSA build provenance attestations.

---

### Community Package Managers

#### Homebrew (macOS & Linux)
```bash
brew install githubuser2777/tap/lavaterm
```

#### Arch Linux (AUR)
```bash
# Using yay
yay -S lavaterm

# Using paru
paru -S lavaterm
```

---

### Linux Installers

#### Option A: Portable AppImage (Universal Linux)

The AppImage runs on any modern Linux distribution without requiring root privileges or package managers.

1. **Download** the latest `ZenLavaTerm-v<VERSION>-linux-x86_64.AppImage` from [GitHub Releases](https://github.com/githubuser2777/ZenLavaTerm/releases).
2. **Make it executable**:
   ```bash
   chmod +x ZenLavaTerm-v*-linux-x86_64.AppImage
   ```
3. **Run**:
   ```bash
   ./ZenLavaTerm-v*-linux-x86_64.AppImage
   ```

#### Option B: Debian / Ubuntu Package (`.deb`)

The `.deb` package installs the `lavaterm` binary system-wide to `/usr/bin/lavaterm`, registers desktop integration, and installs documentation and licenses.

1. **Download** the latest `ZenLavaTerm-v<VERSION>-linux-x86_64.deb` from [GitHub Releases](https://github.com/githubuser2777/ZenLavaTerm/releases).
2. **Install using `apt` or `dpkg`**:
   ```bash
   # Using apt (automatically resolves any dependencies)
   sudo apt install ./ZenLavaTerm-v*-linux-x86_64.deb

   # Or using dpkg
   sudo dpkg -i ZenLavaTerm-v*-linux-x86_64.deb
   ```
3. **Launch**:
   ```bash
   lavaterm
   ```

---

### Windows Installers

#### Windows Installer (`.msi`)

The `.msi` installer provides a standard Windows installation experience.

1. **Download** `ZenLavaTerm-v<VERSION>-windows-x86_64.msi` from [GitHub Releases](https://github.com/githubuser2777/ZenLavaTerm/releases).
2. **Run the installer**: Double-click the `.msi` file and follow the setup wizard.
3. **Features**:
   - Installs `lavaterm.exe` to `Program Files\ZenLavaTerm`.
   - Adds the installation directory to system `PATH` for immediate use from Windows Terminal, PowerShell, or Command Prompt.
   - Registers standard Add/Remove Programs entry for clean uninstallation.
4. **Launch**: Open Windows Terminal or PowerShell and run:
   ```powershell
   lavaterm
   ```

---

### macOS Installers

#### Apple Disk Image (`.dmg`)

The `.dmg` contains a universal application bundle supporting both Apple Silicon (M1/M2/M3/M4) and Intel Macs.

1. **Download** `ZenLavaTerm-v<VERSION>-macos-universal.dmg` from [GitHub Releases](https://github.com/githubuser2777/ZenLavaTerm/releases).
2. **Mount the disk image**: Double-click the `.dmg` file.
3. **Install**: Drag the **ZenLavaTerm** application icon into the **Applications** folder shortcut.
4. **Launch**: Run `ZenLavaTerm` from Applications or execute the binary directly from your terminal:
   ```bash
   /Applications/ZenLavaTerm.app/Contents/MacOS/lavaterm
   ```

> **Note on Signing & Notarization:** Official macOS builds are currently unsigned community binaries. On macOS Ventura/Sonoma/Sequoia, if Gatekeeper warns about an unidentified developer, open **System Settings → Privacy & Security** and click **Open Anyway**, or run `xattr -cr /Applications/ZenLavaTerm.app`.


---

## 3. Build From Source

Users who prefer building locally or are using other distributions (such as Arch Linux, Fedora, NixOS, Alpine) can build directly with Cargo:

### Using Cargo (Recommended for Rust Users)

```bash
cargo install --locked --git https://github.com/githubuser2777/ZenLavaTerm.git
```

### Manual Compilation from Git Clone

```bash
# 1. Clone the repository
git clone https://github.com/githubuser2777/ZenLavaTerm.git
cd ZenLavaTerm

# 2. Build optimized release binary
cargo build --release

# 3. Binary location
./target/release/lavaterm
```

### Arch Linux (AUR / PKGBUILD)

Arch Linux users can build using `makepkg` or an AUR helper:

```bash
# Clone repository and build via PKGBUILD
cd packaging/arch
makepkg -si
```

---

## 4. Maintainer Release Process

The ZenLavaTerm CI/CD architecture follows a strict three-tier lifecycle:
1. **Tier 1: Pull Request CI (`.github/workflows/ci.yml`)**: Fast developer validation (fmt, clippy, unit/integration test suites, headless smoke runs, and cross-platform compilation).
2. **Tier 2: Packaging Validation (`.github/workflows/package.yml`)**: Release Candidate testing triggered manually via `workflow_dispatch` or `v*-rc*` tags. Builds and validates all 4 official desktop installers without publishing a public release.
3. **Tier 3: Production Release (`.github/workflows/release.yml`)**: Triggered strictly on production `vX.Y.Z` release tags to build installers, generate checksum manifests and SLSA provenance attestations, and publish the GitHub Release.

### Cutting a New Release:

1. **Update Version**:
   Update `version = "X.Y.Z"` in `Cargo.toml` and document changes in `CHANGELOG.md`.

2. **Validate Locally**:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   cargo build --release
   ```

3. **Optional: Validate Packaging via Release Candidate Tag**:
   ```bash
   git tag -a "vX.Y.Z-rc.1" -m "Release Candidate vX.Y.Z-rc.1"
   git push origin vX.Y.Z-rc.1
   ```
   Or trigger the `Packaging Validation` workflow manually in GitHub Actions (`workflow_dispatch`).

4. **Publish Production Tag**:
   ```bash
   git commit -am "chore(release): bump version to vX.Y.Z"
   git tag -a "vX.Y.Z" -m "Release vX.Y.Z"
   git push origin main --tags
   ```

5. **Automated Release Pipeline Execution**:
   The `.github/workflows/release.yml` workflow triggers on `vX.Y.Z`:
   - Validates that the git tag strictly matches the `Cargo.toml` package version.
   - Builds native Linux x86_64 release binary and packages `.AppImage` and `.deb`.
   - Builds Windows x86_64 release binary and packages `.msi` via WiX.
   - Builds macOS Apple Silicon and Intel targets, merges them into a universal binary, and packages `.dmg`.
   - Generates independent builder checksums and consolidated `SHA256SUMS.txt`.
   - Generates SLSA build provenance attestations.
   - Creates GitHub Release and publishes all 4 verified installer assets.
