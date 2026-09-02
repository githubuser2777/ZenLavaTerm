# ZenLavaTerm Packaging & Community Distribution

This directory contains package manager manifests, recipes, and formulas for distributing **ZenLavaTerm** across platforms.

## Manifest Layout

| Directory / File | Distribution Target | Target OS | Upstream Reference |
|---|---|---|---|
| `packaging/homebrew/lavaterm.rb` | Homebrew Formula | macOS / Linux | [Homebrew Tap](https://brew.sh/) |
| `packaging/aur/PKGBUILD` | Arch User Repository | Arch Linux / Manjaro | [Arch AUR](https://aur.archlinux.org/) |
| `packaging/aur/.SRCINFO` | AUR Source Info Metadata | Arch Linux | `makepkg --printsrcinfo` |
| `packaging/appimage/` | Linux AppImage AppRun & Desktop | Linux x86_64 | Standalone Portable |
| `packaging/debian/` | Debian Control & Postinst | Debian / Ubuntu | `dpkg-deb` |
| `packaging/macos/` | macOS App Bundle Layout | macOS Universal | `lipo` + DMG |

## Release Checksum Synchronization

During development and release candidate preparation, manifests maintain release templates. When cutting an official release tag (`vX.Y.Z`), the release automation runs:

```bash
./scripts/update_package_manifests.sh <VERSION> <DIST_DIR>
```

This script extracts cryptographic SHA-256 hashes from built release assets (source archives, MSI installers, AppImages) and injects them directly into the respective package manager manifests.
