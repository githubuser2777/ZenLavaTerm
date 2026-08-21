# LavaTerm Roadmap

This document outlines the phased milestone progression of LavaTerm. Each phase builds upon the previous phase following strict Definition of Done criteria.

**Current Maintenance State:** v0.11.0 released — Phase 11 Complete (Cross-Platform Expansion & Hardening) — Phase 12 Planned.

---

## Phase Overview

```text
Phase 0: Repository Foundation & Project Bootstrap (Complete - v0.1.0)
   │
Phase 1: Simulation Core & Metaball Field (Complete - v0.1.0)
   │
Phase 2: Virtual Framebuffer & RGB Color Pipeline (Complete - v0.1.0)
   │
Phase 3: Half-Block Terminal Renderer & Live Animation (MVP) (Complete - v0.1.0)
   │
Phase 4: Multi-Renderer Architecture (Block, Braille) (Complete - v0.4.0)
   │
Phase 5: TOML Configuration Engine & CLI Overrides (Complete - v0.4.0)
   │
Phase 6: System-Reactive Signals (CPU, RAM, Battery) (Complete - v0.6.0)
   │
Phase 7: Audio-Reactive Pipeline (FFT / Beat Generator) (Complete - v0.8.0)
   │
Phase 8: Theme Engine (pywal, wallust, ANSI presets) (Complete - v0.9.0)
   │
Phase 9: Multiplexer & Widget Mode (tmux, zellij) (Complete - v0.10.0)
   │
Phase 10: Interactive Mode (Mouse, Keyboard Ripple) (Complete - v0.11.0)
   │
Phase 11: Cross-Platform Expansion & Hardening (Complete - v0.11.0)
   │
Phase 12: Performance Optimization, Native Audio Capture & V1.0 Release (Planned - Next)
```

---

## Phase Details

### Phase 0: Repository Foundation (Complete - v0.1.0)
- Clean repository structure, licensing, contributing guidelines, and CI pipelines.
- Complete modular architectural skeleton (`core`, `render`, `config`, `input`).
- Executable binary with headless simulation test harness.

### Phase 1: Simulation Core (Complete - v0.1.0)
- Blob state model (`position`, `velocity`, `radius`, `temperature`).
- Scalar field evaluation with configurable potential falloff kernel.
- Buoyancy, gravity, viscous drag, and Brownian thermal drift physics.
- 100% deterministic unit tests using seeded PRNG.

### Phase 2: Virtual Canvas & Color Pipeline (Complete - v0.1.0)
- In-memory 2D `VirtualFramebuffer` decoupled from terminal dimensions.
- RGB color interpolation, multi-stop gradient calculation, and palette mapping.
- Double-buffering data structures and dirty-cell diffing algorithms.

### Phase 3: Terminal Renderer (MVP Target) (Complete - v0.1.0)
- Crossterm backend initialization: raw mode, alternate screen, cursor hide/restore.
- High-resolution half-block (`▀` / `▄`) True Color rendering engine.
- Batched stdout stream writing for 60 FPS flicker-free output.
- Terminal resize handling and graceful shutdown hooks.

### Phase 4: Multi-Renderer Support (Complete - v0.4.0)
- Pluggable `Renderer` trait abstraction.
- Full-block (`█`) and Braille dot matrix rendering engines.
- CLI switch: `lavaterm --renderer [halfblock|block|braille]`.

### Phase 5: Configuration & CLI (Complete - v0.4.0)
- Full TOML configuration file parsing with fallback to defaults.
- Configuration directory auto-discovery (`~/.config/lavaterm/config.toml`).
- CLI argument overrides for all configuration keys.

### Phase 6: System-Reactive Signals (Complete - v0.6.0)
- Modular OS signal provider for CPU load, RAM usage, and battery status.
- Normalized signal mapping to simulation turbulence and blob convection speed.
- Native Linux `/proc` and `/sys` provider with cross-platform fallback provider.

### Phase 7: Audio-Reactive Simulation (Complete - v0.8.0)
- Zero-dependency Cooley-Tukey Radix-2 FFT spectrum analyzer with Hann windowing.
- Decoupled `AudioSignals` domain structure and `AudioProvider` trait.
- Procedural `SyntheticAudioGenerator` default provider and `PcmRingBuffer` / `LiveAudioProvider` infrastructure.
- Frequency band mapping into convective buoyancy surges, fluid turbulence, and micro-perturbation jitter.

### Phase 8: Theme Engine Integration (Complete - v0.9.0)
- Built-in curated presets (`lava`, `ocean`, `cyberpunk`, `synthwave`, `nord`, `forest`, `monochrome`, `matrix`, `sunset`, `dracula`, `catppuccin`, `tokyo-night`).
- Zero-dependency extraction from Pywal (`~/.cache/wal/colors.json`, flat `colors`) and Wallust (`~/.cache/wallust/colors.json`, `nix-colors.json`).
- Auto-detection engine (`--theme auto`) querying active desktop schemes with robust fallback.
- Custom user theme file support (`.json` and `.toml`).
- Full CLI `--theme <name|auto|path>` and TOML `[theme]` configuration integration.

### Phase 9: Multiplexer & Compact Modes (Complete - v0.10.0)
- Zero-dependency `tmux` and `zellij` environment detection (`src/widget/multiplexer.rs`).
- Adaptive compact geometry scaler with profile-based parameter calculation (`src/widget/compact.rs`).
- Single-shot ANSI True Color status bar serializer (`render_snapshot` in `src/widget/snapshot.rs`).
- Interactive in-place inline rendering mode (`--inline`) without alternate screen.
- Policy resolution engine (`src/widget/policy.rs`) managing precedence, FPS defaults, and conflict validation.
- CLI flags (`--fps`, `--compact`, `--widget`, `--inline`, `--snapshot`, `--width`, `--height`) and TOML `[widget]` configuration section.

### Phase 10: Interactive Physics & Input Mode (Complete - v0.11.0)
- **10.1 Mouse click → Shockwave**: Left-click radial explosive impulse pushing blobs outward with soft-core inverse-distance falloff and thermal excitation (`apply_shockwave` in `src/core/interaction.rs`).
- **10.2 Mouse drag → Stirring**: Fluid stirring from mouse drag motion vectors transferring directional velocity within an influence radius (`apply_stir` in `src/core/interaction.rs`, `MouseTracker` in `src/input/mouse.rs`).
- **10.3 Keyboard → Ripple**: Character keypress detection injecting harmonic acoustic wave ripples and thermal fluctuations (`apply_ripple` in `src/core/interaction.rs`, `map_key_event_with_ripple` in `src/input/keyboard.rs`).
- **Terminal Coordinate Normalizer**: Inverted coordinate mapper converting terminal grid cells to normalized $[0.0, 1.0]$ simulation coordinates (`src/input/coords.rs`).
- **Fail-Safe Lifecycle**: Mouse capture initialization with panic hook, error handling, and signal safety (`src/main.rs`).
- **Configuration & CLI**: CLI flags (`--no-mouse`, `--no-ripple`, `--shockwave-force`, `--stir-force`) and TOML `[interaction]` section.

### Phase 11: Cross-Platform Expansion & Hardening (Complete - v0.11.0)
- **Objective**: Full first-class cross-platform support across Linux, Windows, and macOS with zero external runtime C dependencies and guaranteed graceful degradation.
- **Native System Providers**:
  - `WindowsSystemProvider` (`src/reactive/windows.rs`) using zero-dependency Win32 APIs: `GetSystemTimes` for CPU tick deltas, `GlobalMemoryStatusEx` for physical RAM utilization, `GetSystemPowerStatus` for battery and AC status, and `GetProcessIoCounters` for process I/O activity.
  - `MacOSSystemProvider` (`src/reactive/macos.rs`) using Mach kernel subsystem APIs: `host_statistics64` with `HOST_CPU_LOAD_INFO` for CPU load ticks and `HOST_VM_INFO64` for memory pages.
  - Dynamic factory `default_system_provider()` (`src/reactive/mod.rs`) auto-instantiating the platform-native provider or `MockSystemProvider` on unsupported environments.
- **Platform-Specific Signal Handling & Teardown**:
  - Native Windows console control routine (`SetConsoleCtrlHandler`) catching `CTRL_C_EVENT` and `CTRL_CLOSE_EVENT`.
  - Unix signal handlers for `SIGINT` and `SIGTERM` via `signal-hook`.
  - Dynamic panic hooks and teardown functions aware of fullscreen vs inline modes, reliably disabling mouse capture, leaving alternate screens, and showing the cursor.
- **Cross-Platform Configuration & Theme Cache Discovery**:
  - Multi-platform configuration hierarchy (`src/config/mod.rs`): XDG (`$XDG_CONFIG_HOME`), Windows `%APPDATA%\lavaterm\config.toml` and `%USERPROFILE%`, macOS `$HOME/Library/Application Support/lavaterm/config.toml`, and Unix `~/.config/lavaterm/config.toml`.
  - Multi-platform theme cache discovery in Pywal and Wallust (`LOCALAPPDATA`, `APPDATA`, `USERPROFILE`, `XDG_CACHE_HOME`, `HOME`).
- **Three-Tier CI/CD & Minimal Desktop Release Packaging**:
  - Production release matrix: Linux AppImage (`x86_64`), Linux DEB (`x86_64`), Windows MSI (`x86_64` via WiX), and macOS Universal DMG (`arm64` + `x86_64` universal binary).
  - Consolidated `SHA256SUMS.txt` manifest, individual `.sha256` checksums, and SLSA build provenance attestations.
  - Three-tier workflow architecture: PR CI (`ci.yml`), Release Candidate Packaging (`package.yml`), and Strict SemVer Production Release (`release.yml`).
- **Testing & Validation**:
  - 120 automated tests (105 unit + 15 integration tests), including cross-platform provider contracts, lifecycle transitions, theme discovery, and headless smoke testing across Ubuntu, macOS, and Windows runners.
- **Remaining Limitations**:
  - Live hardware audio capture backends (WASAPI capture on Windows, CoreAudio on macOS, native PipeWire on Linux) remain synthetic/stream-based and are scheduled for Phase 12.

### Phase 12: Performance Optimization, Native Audio Capture & V1.0 Release (Completed & Released — Milestone #1)
- **Objective**: Implement native live audio capture across Windows, Linux, and macOS, evidence-driven field & rasterization performance optimizations, package manager distribution manifests, and v1.0.0 production stabilization.
- **GitHub Milestone**: `Phase 12 — Performance, Native Audio & V1.0` (Milestone #1)
- **Status**: Complete — All 12 issues resolved, 132 tests passing, v1.0.0 released.
- **Issue Breakdown**:
  - `Issue 12.0` (#45): Architecture, Performance Baseline & Phase 12 Inception (Closed)
  - `Issue 12.1` (#46): Native Audio Architecture, Dynamic Provider Contract & Ring Buffer Hardening (Closed)
  - `Issue 12.2` (#47): Windows Native Audio Capture (WASAPI Loopback & Device Stream) (Closed)
  - `Issue 12.3` (#48): Linux Native Audio Capture (ALSA / PipeWire Stream Capture) (Closed)
  - `Issue 12.4` (#49): macOS Native Audio Capture (CoreAudio Stream & Permission Handling) (Closed)
  - `Issue 12.5` (#50): Unified Cross-Platform Audio Runtime, CLI `--audio-device` & Dynamic Fallback (Closed)
  - `Issue 12.6` (#51): Micro-Benchmark Expansion, Allocation Profiling & Hotspot Analysis (Closed)
  - `Issue 12.7` (#52): High-Performance Scalar Field & Framebuffer Rasterization Optimizations (Closed)
  - `Issue 12.8` (#53): Community Package Manager Distribution (Homebrew, AUR, Scoop, Winget) (Closed)
  - `Issue 12.9` (#54): V1.0 API Freeze, Configuration Migration Engine & Security Hardening (Closed)
  - `Issue 12.10` (#55): V1.0 Release Candidate Validation & Documentation Sync (In Progress)
  - `Issue 12.11` (#56): ZenLavaTerm v1.0.0 Production Release & Transition (Pending)


