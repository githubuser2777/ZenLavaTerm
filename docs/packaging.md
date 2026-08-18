# LavaTerm Arch Linux Packaging & Distribution Guide

This document outlines how LavaTerm is packaged, distributed, and installed on **Arch Linux** and Arch-based distributions (Manjaro, EndeavourOS, Garuda, etc.).

---

## 1. Distribution Channels

LavaTerm provides two installation tracks for Arch Linux users:

```text
┌──────────────────────────────────────────────────────────────┐
│                    Arch Linux Distribution                   │
└──────────────┬────────────────────────────────┬──────────────┘
               │                                │
               ▼                                ▼
┌──────────────────────────────┐ ┌──────────────────────────────┐
│  Track 1: Pre-built Package  │ │   Track 2: Build From Source │
│    (Tải về dùng ngay)        │ │       (Tự biên dịch)         │
├──────────────────────────────┤ ├──────────────────────────────┤
│ • .pkg.tar.zst từ Release    │ │ • AUR package: lavaterm      │
│ • AUR package: lavaterm-bin  │ │ • makepkg -si                │
│ • Cài đặt trong 1s           │ │ • cargo install / local pkg  │
│ • Không cần Rust toolchain   │ │ • Tối ưu theo vi kiến trúc   │
└──────────────────────────────┘ └──────────────────────────────┘
```

---

## 2. Track 1: Pre-built Binary Packages (Tải về dùng ngay)

### Method A: Download `.pkg.tar.zst` from GitHub Releases

Every release tag (`v*`) automatically builds a standalone native Arch Linux package:

```bash
# 1. Download release package (replace <VERSION> with target version, e.g. 0.11.0)
wget https://github.com/githubuser2777/ZenLavaTerm/releases/download/v<VERSION>/lavaterm-<VERSION>-1-x86_64.pkg.tar.zst

# 2. Install using pacman
sudo pacman -U lavaterm-<VERSION>-1-x86_64.pkg.tar.zst
```

### Method B: Install `lavaterm-bin` via AUR helper

Using `yay` or `paru`:

```bash
# With yay
yay -S lavaterm-bin

# With paru
paru -S lavaterm-bin
```

---

## 3. Track 2: Build From Source (Tự biên dịch)

### Method A: Build from Source via AUR helper

```bash
yay -S lavaterm
```

### Method B: Build using `makepkg` & PKGBUILD

```bash
git clone https://github.com/githubuser2777/ZenLavaTerm.git
cd ZenLavaTerm/packaging/arch
makepkg -si
```

### Method C: Local automated packaging script

The repository provides a helper script `scripts/package_arch.sh`:

```bash
# Build the .pkg.tar.zst package in target/arch_pkg/
./scripts/package_arch.sh

# Build and install immediately
./scripts/package_arch.sh --install
```

### Method D: Install via Cargo

```bash
cargo install --locked --git https://github.com/githubuser2777/ZenLavaTerm.git
```

---

## 4. Package Maintenance & AUR Release Checklist

When cutting a new release:
1. Bump `version = "x.y.z"` in `Cargo.toml`.
2. Update `pkgver=x.y.z` in `packaging/arch/PKGBUILD` and `packaging/arch/PKGBUILD.bin`.
3. Regenerate `.SRCINFO`:
   ```bash
   cd packaging/arch && makepkg --printsrcinfo > .SRCINFO
   ```
4. Push tag `v*` to GitHub to trigger `.github/workflows/release.yml`.
