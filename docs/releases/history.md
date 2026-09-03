# Release History Overview

This document summarizes the major milestone releases of ZenLavaTerm. For granular line-by-line changelog entries and commit references, see [CHANGELOG.md](file:///home/skids/Documents/code/ZenLavaTerm/CHANGELOG.md).

---

## Release Milestones

### [v1.0.1] - 2026-09-03 (Hygiene & Optimization Patch)
- **Highlights**:
  - Test fixture isolation: moved `MockAudioStreamFeeder` (~230 LOC) into `tests/common/mod.rs`.
  - Removed direct `thiserror` dependency in favor of standard `std::error::Error`.
  - Replaced manual AST rewriting migration engine with native Serde field aliases.
  - Upgraded dependencies (`crossterm 0.29`, `cpal 0.18`, `criterion 0.8`).
  - Total automated tests: 135 passing (112 unit + 23 integration).

### [v1.0.0] - 2026-08-21 (Phase 12: Audio Architecture & Production Release)
- **Highlights**:
  - Full cross-platform audio streaming capture via `cpal` (ALSA, WASAPI, CoreAudio).
  - SPSC lock-free Seqlock ring buffer (`PcmRingBuffer`) with guaranteed tear-free snapshot coherence.
  - Radix-2 Cooley-Tukey FFT spectrum analyzer with Hann windowing.
  - Verified >5,000 FPS rasterization throughput in Criterion micro-benchmarks.
  - Desktop packaging: Linux AppImage & DEB, Windows MSI, macOS Universal DMG.

### [v0.11.0] - 2026-08-19 (Phase 11: Cross-Platform Expansion)
- **Highlights**:
  - Native Windows telemetry provider via Win32 APIs (`GetSystemTimes`, `GlobalMemoryStatusEx`).
  - Native macOS telemetry provider via Mach kernel subsystem (`host_statistics64`).
  - Cross-platform configuration and theme path discovery (XDG, AppData, Library).
  - Windows console control signal handling (`CTRL_C_EVENT`).
