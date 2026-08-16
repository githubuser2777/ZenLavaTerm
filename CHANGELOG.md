# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-08-16 — Phase 8: Theme Engine & Desktop Ricing

### Added
- Modular `theme` engine decoupled from terminal rendering and simulation physics (`src/theme/`).
- 12 curated high-aesthetic built-in presets: `lava` (default), `ocean`, `cyberpunk`, `synthwave`, `nord`, `forest`, `monochrome`, `matrix`, `sunset`, `dracula`, `catppuccin`, and `tokyo-night`.
- Dynamic Linux desktop color extraction from Pywal (`~/.cache/wal/colors.json` and flat `colors`).
- Dynamic Linux desktop color extraction from Wallust (`~/.cache/wallust/colors.json`, `nix-colors.json`).
- Auto-detection engine (`--theme auto`) querying active desktop schemes with robust fallback.
- Custom user theme file parser supporting arbitrary JSON and TOML formats.
- CLI argument `-t`, `--theme <NAME|AUTO|PATH>` and TOML `[theme]` configuration section.
- Comprehensive unit and integration test suite covering theme resolution, JSON/TOML parsing, and preset rasterization.

## [0.8.0] - 2026-08-16 — Phase 7: Audio-Reactive Mode

### Added
- Zero-dependency Cooley-Tukey Radix-2 Fast Fourier Transform (FFT) with Hann windowing (`SpectrumAnalyzer`).
- Frequency energy spectrum band extractor partitioning into `bass` (20-250 Hz), `mid` (250-4000 Hz), and `treble` (4000-20000 Hz).
- Decoupled `AudioSignals` domain structure and `AudioProvider` trait.
- Thread-safe circular `PcmRingBuffer` and `LiveAudioProvider` decoupling audio capture from 60 FPS rendering.
- `SyntheticAudioGenerator` procedural beat generator for headless testing and demos.
- Dynamic physical fluid modulation in `Simulation`: bass kick upward convective surges, midrange fluid turbulence, treble kinetic jitter.
- CLI flag `--audio` and TOML configuration section `[audio]`.
- Integration and unit test suites verifying FFT frequency isolation and audio-reactive simulation.

## [0.6.0] - 2026-08-16 — Phase 6: System-Reactive Ambient Observability

### Added
- Decoupled `SystemSignals` domain model normalizing CPU, Memory, Battery, and I/O metrics in `[0.0, 1.0]`.
- Trait-based `SystemProvider` abstraction and deterministic `MockSystemProvider`.
- Native zero-dependency Linux metrics collector (`LinuxSystemProvider`) reading `/proc/stat`, `/proc/meminfo`, `/proc/diskstats`, and `/sys/class/power_supply` with graceful fallback handling.
- Dynamic physical fluid modulation in `Simulation`: CPU turbulence, RAM blob scaling, and Battery buoyancy.
- CLI switch `--system` and TOML configuration section `[reactive]` with customizable polling interval.
- Headless and interactive event loops integrated with background metric poller.
- Comprehensive unit and integration test coverage for signal gathering and fluid reaction.

## [0.4.0] - 2026-08-16 — Phase 4: Multi-Renderer Architecture & Performance Benchmarks

### Added
- Braille dot matrix (`U+2800`..`U+28FF`) 2x4 sub-pixel terminal renderer (`BrailleRenderer`).
- Multi-renderer CLI selection switch: `lavaterm --renderer [halfblock|block|braille]`.
- Dynamic aspect-ratio and virtual canvas resizing for sub-cell rendering grids.
- Criterion micro-benchmark suite (`benches/field_and_render.rs`) for field evaluation and ANSI renderer serialization.
- Automated multi-platform GitHub Release workflow (`.github/workflows/release.yml`) for Linux, macOS, and Windows with SHA256 checksums.
- End-to-end integration tests covering all 3 terminal renderers.

## [0.1.0] - 2026-08-16 — Phase 0 & Phase 1-3: Core Simulation & Terminal MVP

### Added
- Initial project bootstrap, repository foundation, and community health guidelines.
- Decoupled unidirectional architecture: `core`, `render`, `config`, and `input`.
- 2D Metaball continuous scalar field evaluation and convective fluid physics.
- In-memory `VirtualFramebuffer` abstraction with 24-bit True Color palette interpolation.
- Unicode Half-Block (`▀`) and Full-Block (`█`) terminal renderers.
- TOML configuration parser with default fallback and validation.
- Interactive terminal event loop with crossterm, raw mode, alternate screen, and panic recovery hooks.
- Cross-platform GitHub Actions CI workflow for Linux, macOS, and Windows.
