# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
