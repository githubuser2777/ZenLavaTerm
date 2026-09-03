# Getting Started with ZenLavaTerm Development

This guide covers everything you need to set up a local development environment, build the binary, and run tests.

---

## 1. Prerequisites

### Rust Toolchain
- **Rust Stable**: 1.85 or later (2021 edition).
- Install via `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update stable
  rustup component add rustfmt clippy
  ```

### System Audio Libraries (Linux only)
On Linux systems, `cpal` requires the ALSA development headers and `pkg-config`:
```bash
# Debian / Ubuntu / Pop!_OS
sudo apt-get update && sudo apt-get install -y libasound2-dev pkg-config

# Arch Linux / Manjaro
sudo pacman -S alsa-lib pkgconf

# Fedora / RHEL
sudo dnf install alsa-lib-devel pkgconf-pkg-config
```
*(On Windows and macOS, native audio APIs WASAPI and CoreAudio require no external C packages.)*

---

## 2. Cloning & Building

```bash
# Clone the repository
git clone https://github.com/githubuser2777/ZenLavaTerm.git
cd ZenLavaTerm

# Build in debug mode
cargo build

# Fast sanity typecheck
cargo check --all-targets --all-features

# Build optimized release binary
cargo build --release
```

The compiled binary will be located at `target/release/lavaterm`.

---

## 3. Running ZenLavaTerm

```bash
# Run with default interactive half-block renderer
cargo run

# Run with Braille renderer at 60 FPS
cargo run -- --renderer braille --fps 60

# Run in audio-reactive mode
cargo run -- --audio

# Run in system telemetry reactive mode
cargo run -- --system

# Run a headless 30-frame verification test (no TTY required)
cargo run -- --headless --frames 30

# Render a single ANSI frame snapshot to stdout
cargo run -- --snapshot
```
