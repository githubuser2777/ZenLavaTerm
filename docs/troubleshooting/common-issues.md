# Troubleshooting & FAQ

This document addresses common runtime errors, build issues, and platform quirks encountered with ZenLavaTerm.

---

## 1. Audio Capture & Loopback

### Issue: "Audio capture unavailable; falling back to synthetic generator"
- **Cause**: The application cannot access a live microphone or audio loopback device.
- **Resolution**:
  - Check available audio devices:
    ```bash
    lavaterm --list-audio-devices
    ```
  - Specify a particular device explicitly:
    ```bash
    lavaterm --audio --audio-device "<DEVICE_NAME>"
    ```
  - On Linux: Ensure user belongs to the `audio` group (`sudo usermod -aG audio $USER`).
  - On macOS: Ensure Terminal or iTerm2 has Microphone access permissions in *System Settings -> Privacy & Security -> Microphone*.

### Issue: Windows WASAPI loopback produces no visualization
- **Cause**: On Windows, WASAPI loopback captures the shared output render mix. When no application is outputting audio, the Windows audio engine suppresses buffer events.
- **Resolution**: Play background music or media through your default speakers/headphones while running `lavaterm --audio`.

---

## 2. Terminal Rendering & Colors

### Issue: Colors look washed out or inverted
- **Cause**: Terminal emulator lacks 24-bit True Color (DirectColor) support.
- **Resolution**:
  - Verify terminal TrueColor support:
    ```bash
    echo $COLORTERM
    ```
    Should output `truecolor` or `24bit`.
  - Recommended terminals: Alacritty, Kitty, WezTerm, Ghostty, iTerm2, Windows Terminal, Foot.
  - If using a constrained terminal, use the full-block renderer:
    ```bash
    lavaterm --renderer block
    ```

### Issue: Terminal cursor remains hidden or corrupted after an abrupt exit
- **Cause**: Shell session terminated abnormally without executing cleanup handlers.
- **Resolution**: Run `reset` or `stty sane`:
  ```bash
  stty sane
  ```

---

## 3. Build & Compilation Issues

### Issue: Linux compilation fails with `pkg-config` or `alsa` missing
- **Cause**: Missing ALSA C development libraries.
- **Resolution**:
  ```bash
  # Debian/Ubuntu
  sudo apt-get install -y libasound2-dev pkg-config

  # Fedora
  sudo dnf install alsa-lib-devel pkgconf-pkg-config
  ```

### Issue: `cargo test` fails with linker error on macOS
- **Cause**: Missing Xcode Command Line Tools.
- **Resolution**:
  ```bash
  xcode-select --install
  ```
