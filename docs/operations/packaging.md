# Packaging & Desktop Distribution Guide

ZenLavaTerm supports native desktop packaging across Linux, Windows, and macOS, along with community package manager manifests for Arch Linux AUR and Homebrew.

---

## 1. Supported Packages & Artifacts

| Platform | Format | Build Command / Script | Artifact Output |
|---|---|---|---|
| **Linux** | AppImage | `scripts/package_linux.sh` | `ZenLavaTerm-v<VERSION>-linux-x86_64.AppImage` |
| **Linux** | Debian (`.deb`) | `scripts/package_linux.sh` (`dpkg-deb`) | `ZenLavaTerm-v<VERSION>-linux-x86_64.deb` |
| **Windows** | MSI Installer | `scripts/package_windows.ps1` (WiX 3.11/3.14) | `ZenLavaTerm-v<VERSION>-windows-x86_64.msi` |
| **macOS** | Universal DMG | `scripts/package_macos.sh` (`lipo` + `hdiutil`) | `ZenLavaTerm-v<VERSION>-macos-universal.dmg` |
| **Arch Linux** | AUR / PKGBUILD | `scripts/update_package_manifests.sh` | `packaging/aur/PKGBUILD` |
| **macOS / Linux** | Homebrew Formula | `scripts/update_package_manifests.sh` | `packaging/homebrew/lavaterm.rb` |

---

## 2. Local Packaging Procedures

### 2.1 Linux (AppImage & DEB)
Requirements: `dpkg-deb`, `file`.
```bash
# 1. Build release binary
cargo build --release --target x86_64-unknown-linux-gnu

# 2. Package into dist/
./scripts/package_linux.sh target/x86_64-unknown-linux-gnu/release/lavaterm dist
```

### 2.2 Windows (MSI via WiX)
Requirements: Windows PowerShell, WiX Toolset v3.11 or v3.14 (`candle.exe`, `light.exe`).
```powershell
# 1. Build release binary
cargo build --release --target x86_64-pc-windows-msvc

# 2. Build MSI installer into dist\
powershell -ExecutionPolicy Bypass -File scripts\package_windows.ps1 -SourceBinaryDir "target\x86_64-pc-windows-msvc\release" -OutputDir "dist"
```

### 2.3 macOS (Universal DMG)
Requirements: macOS with Xcode command-line tools (`lipo`, `hdiutil`).
```bash
# 1. Build Apple Silicon and Intel targets
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# 2. Merge targets via lipo
mkdir -p target/universal/release
lipo -create -output target/universal/release/lavaterm \
  target/aarch64-apple-darwin/release/lavaterm \
  target/x86_64-apple-darwin/release/lavaterm

# 3. Create DMG disk image
./scripts/package_macos.sh universal target/universal/release/lavaterm dist
```

---

## 3. Package Manifest Synchronization

When releasing a new version, package manifests must be synchronized with the release tarball's SHA256 checksum:

```bash
./scripts/update_package_manifests.sh <VERSION>
```
This script downloads the tagged release source tarball, computes its SHA256 hash, and injects the version and hash into `packaging/homebrew/lavaterm.rb`, `packaging/aur/PKGBUILD`, and `packaging/aur/.SRCINFO`.
