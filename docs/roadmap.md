# LavaTerm Roadmap

This document outlines the phased milestone progression of LavaTerm. Each phase builds upon the previous phase following strict Definition of Done criteria.

**Current Maintenance State:** v0.11.0 released — Phase 10 Complete (Interactive Physics & Input Mode) — Phase 11 Planned.

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
Phase 11: Cross-Platform Hardening (Windows, macOS) (Planned - Next)
   │
Phase 12: Performance Profiling, Packaging & V1.0 Release (Planned)
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

### Phase 11: Cross-Platform Hardening (Planned - Next)
- Windows WASAPI and macOS CoreAudio integration without polluting core simulation.

### Phase 12: V1.0 Polish & Packaging (Planned)
- Micro-benchmarking with Criterion.rs, memory footprint minimization, AUR / Homebrew packaging.
