# LavaTerm Roadmap

This document outlines the phased milestone progression of LavaTerm. Each phase builds upon the previous phase following strict Definition of Done criteria.

---

## Phase Overview

```text
Phase 0: Repository Foundation & Project Bootstrap (Current)
   │
Phase 1: Simulation Core & Metaball Field
   │
Phase 2: Virtual Framebuffer & RGB Color Pipeline
   │
Phase 3: Half-Block Terminal Renderer & Live Animation (MVP)
   │
Phase 4: Multi-Renderer Architecture (Block, Braille)
   │
Phase 5: TOML Configuration Engine & CLI Overrides
   │
Phase 6: System-Reactive Signals (CPU, RAM, Battery)
   │
Phase 7: Audio-Reactive Pipeline (PipeWire / FFT)
   │
Phase 8: Theme Engine (pywal, wallust, ANSI)
   │
Phase 9: Multiplexer & Widget Mode (tmux, zellij)
   │
Phase 10: Interactive Mode (Mouse, Keyboard Ripple)
   │
Phase 11: Cross-Platform Hardening (Windows, macOS)
   │
Phase 12: Performance Profiling, Packaging & V1.0 Release
```

---

## Phase Details

### Phase 0: Repository Foundation (Current)
- Clean repository structure, licensing, contributing guidelines, and CI pipelines.
- Complete modular architectural skeleton (`core`, `render`, `config`, `input`).
- Executable binary with headless simulation test harness.

### Phase 1: Simulation Core
- Blob state model (`position`, `velocity`, `radius`, `temperature`).
- Scalar field evaluation with configurable potential falloff kernel.
- Buoyancy, gravity, viscous drag, and Brownian thermal drift physics.
- 100% deterministic unit tests using seeded PRNG.

### Phase 2: Virtual Canvas & Color Pipeline
- In-memory 2D `VirtualFramebuffer` decoupled from terminal dimensions.
- RGB color interpolation, multi-stop gradient calculation, and palette mapping.
- Double-buffering data structures and dirty-cell diffing algorithms.

### Phase 3: Terminal Renderer (MVP Target)
- Crossterm backend initialization: raw mode, alternate screen, cursor hide/restore.
- High-resolution half-block (`▀` / `▄`) True Color rendering engine.
- Batched stdout stream writing for 60 FPS flicker-free output.
- Terminal resize handling and graceful shutdown hooks.

### Phase 4: Multi-Renderer Support
- Pluggable `Renderer` trait abstraction.
- Full-block (`█`) and Braille dot matrix rendering engines.
- CLI switch: `lavaterm --renderer [halfblock|block|braille]`.

### Phase 5: Configuration & CLI
- Full TOML configuration file parsing with fallback to defaults.
- Configuration directory auto-discovery (`~/.config/lavaterm/config.toml`).
- CLI argument overrides for all configuration keys.

### Phase 6: System-Reactive Signals
- Modular OS signal provider for CPU load, RAM usage, and battery status.
- Normalized signal mapping to simulation turbulence and blob convection speed.
- Mock providers for cross-platform unit and integration testing.

### Phase 7: Audio-Reactive Simulation
- Asynchronous PCM audio stream capture (PipeWire on Linux).
- Real-time FFT analysis into low/mid/high frequency bands.
- Decoupled signal dispatch into fluid buoyancy and surface ripples.

### Phase 8: Theme Engine Integration
- Dynamic color extraction from pywal, wallust, and standard terminal 16-color ANSI palettes.

### Phase 9: Multiplexer & Compact Modes
- Low-overhead widget mode for `tmux` status bars and `zellij` panes.

### Phase 10: Interactive Physics
- Mouse click shockwaves, drag stirring, and keyboard ripple interactions.

### Phase 11: Cross-Platform Hardening
- Windows WASAPI and macOS CoreAudio integration without polluting core simulation.

### Phase 12: V1.0 Polish & Packaging
- Micro-benchmarking with Criterion.rs, memory footprint minimization, AUR / Homebrew packaging.
