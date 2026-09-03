# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-21 — Phase 12: Audio Architecture, High Performance & V1.0 Release Candidate

### Added
- Decoupled cross-platform native audio streaming architecture (`LiveAudioProvider` in `src/audio/capture.rs`) powered by `cpal`, implementing real cross-platform hardware audio capture (WASAPI on Windows, ALSA on Linux, CoreAudio on macOS).
- Runtime audio stream fallback and live resumption: `stream_alive: Arc<AtomicBool>` shared across native CPAL error callbacks and `LiveAudioProvider::poll_signals()`, gracefully falling back to `SyntheticAudioGenerator(bpm)` upon hardware disconnection and automatically resuming live processing upon stream recovery.
- Hardware audio frame stream simulator (`MockAudioStreamFeeder` in `src/audio/provider.rs`) for continuous real-time audio testing across f32, i16, and u16 formats, hardware disconnect/reconnect transitions, buffer overrun/underrun resilience, and wrap-around snapshot coherence.
- SPSC Lock-Free circular `PcmRingBuffer` in `src/audio/ring_buffer.rs` utilizing atomic primitives (`AtomicU32`, `AtomicUsize`) paired with a 64-bit sequence lock (`version: AtomicU64`) to guarantee tear-free snapshot consistency during wrap-around under concurrent producer/consumer execution, alongside an atomic CAS spin-guard (`producer_guard`) for multi-producer safety.
- Unified cross-platform audio provider factory (`create_audio_provider`, `create_live_audio_provider`, and `list_audio_devices` in `src/audio/mod.rs`) with guaranteed lifetime retention of active stream backends and graceful synthetic fallback.
- CLI audio flags: `--audio-device <DEVICE>` for selecting specific capture devices and `--list-audio-devices` for enumerating available endpoints.
- TOML configuration `[audio]` schema extension with `device: Option<String>`.
- Multi-channel PCM audio ingestion, stereo-to-mono downmixing, 16-bit integer PCM normalization, and linear sample-rate resampling (`48kHz` <-> `44.1kHz`) in `PcmRingBuffer`.
- Public API freeze and backward-compatible TOML configuration migration engine (`src/config/migrate.rs`) automatically upgrading legacy schemas (`num_blobs`, `renderer_type`, `target_fps`, `smooth_gradient`, `tempo`, `compact_mode`).
- Community package manager manifests and recipes: Homebrew Formula (`packaging/homebrew/lavaterm.rb`) and Arch Linux AUR (`packaging/aur/PKGBUILD` and `.SRCINFO`).
- Fail-closed package manager manifest release synchronization tooling (`scripts/update_package_manifests.sh`).
- Comprehensive micro-benchmark suite (`benches/field_and_render.rs`) with Criterion profiling across scalar field math, multi-resolution rasterization, renderers, Radix-2 FFT window sizes (512, 1024, 2048), linear resampling, lock-free ring buffer under multi-threaded wrap-around contention, and simulation pipeline.


### Optimized & Hardened
- Optimized scalar field potential evaluation and weighted temperature calculations in `src/core/field.rs` with loop invariant hoisting and vectorization-friendly math.
- Optimized framebuffer rasterization in `src/render/mod.rs` with precomputed inverse dimensions and direct contiguous slice indexing, eliminating per-pixel bounds check overhead.
- Optimized `HalfBlockRenderer`, `BlockRenderer`, and `BrailleRenderer` with direct slice indexing and pre-allocated formatting buffers.
- Recorded 50-65% benchmark speedups on rasterization loops (achieving 7,865 FPS on $80 \times 48$ smooth gradient and 15,954 FPS on stepped gradient) and validated lock-free ring buffer throughput under multi-threaded contention, documented in [`docs/benchmarks/benchmark_baseline.md`](docs/benchmarks/benchmark_baseline.md) and [`docs/benchmarks/criterion_baseline.log`](docs/benchmarks/criterion_baseline.log).
- Expanded automated test suite to 143 tests (120 unit tests + 23 integration tests) covering 100% of functional paths.

## [0.11.0] - 2026-08-19 — Phase 11: Cross-Platform Expansion


### Added
- Native Windows system metrics provider (`WindowsSystemProvider` in `src/reactive/windows.rs`) using zero-dependency Win32 APIs: `GetSystemTimes` for CPU tick deltas, `GlobalMemoryStatusEx` for RAM utilization, `GetSystemPowerStatus` for battery and AC line status, and `GetProcessIoCounters` for I/O activity.
- Native macOS system metrics provider (`MacOSSystemProvider` in `src/reactive/macos.rs`) using Mach kernel subsystem APIs: `host_statistics64` with `HOST_CPU_LOAD_INFO` for CPU load and `HOST_VM_INFO64` for memory pages.
- Dynamic cross-platform system provider factory (`default_system_provider()`) instantiating the platform-appropriate provider on Linux, Windows, macOS, or `MockSystemProvider` on other platforms with guaranteed graceful degradation.
- Native Windows console control signal handler (`setup_signal_handler` with `SetConsoleCtrlHandler`) catching `CTRL_C_EVENT` and `CTRL_CLOSE_EVENT` to cleanly disable raw mode, restore alternate screen, and show the cursor before exit.
- Cross-platform configuration path discovery (`src/config/mod.rs`) supporting standard XDG (`$XDG_CONFIG_HOME`), Windows `%APPDATA%\lavaterm\config.toml`, Windows `%USERPROFILE%`, macOS `$HOME/Library/Application Support/lavaterm/config.toml`, and Unix `~/.config/lavaterm/config.toml`.
- Cross-platform desktop theme cache discovery in Pywal and Wallust supporting `LOCALAPPDATA`, `APPDATA`, `USERPROFILE`, `XDG_CACHE_HOME`, and `HOME`.
- Comprehensive cross-platform unit and integration test suite (120 tests total) covering system provider contracts, audio provider contracts, headless execution across platforms, lifecycle transitions, and configuration discovery across all OS path layouts.

### Fixed & Hardened
- Unified Mach kernel `host_statistics64` FFI declarations at module level in `src/reactive/macos.rs` for reliable compilation across Darwin targets.
- Hardened Windows console control handler with `OnceLock` global flag management and safe warning fallbacks.
- Allowed dead code attributes where appropriate for cross-target compilation without warnings under `-D warnings`.
- Enabled graceful fallback for SLSA build provenance attestations in private repository workflows.

### Packaging & CI/CD
- Implemented three-tier CI/CD architecture: PR CI (`ci.yml`), Release Candidate Packaging (`package.yml`), and Strict SemVer Production Release (`release.yml`).
- Streamlined minimal official desktop release matrix: Linux AppImage (`x86_64`), Linux DEB (`x86_64`), Windows MSI (`x86_64` via WiX), and macOS Universal DMG (`arm64` + `x86_64`).
- Added atomic GitHub Release publishing with consolidated `SHA256SUMS.txt` manifest, builder checksum verification, and SLSA build provenance attestations.
- Added native Windows MSI installer packaging script (`scripts/package_windows.ps1`) and WiX Toolset configuration (`wix/main.wxs`).
- Added Linux AppImage and DEB packaging script (`scripts/package_linux.sh`) and desktop integration assets.
- Added macOS Universal DMG packaging script (`scripts/package_macos.sh`) with `lipo` fat binary bundling.

### Documentation
- Updated `README.md`, `docs/roadmap.md`, `docs/architecture.md`, `docs/reactive.md`, `docs/configuration.md`, `docs/packaging.md`, `docs/maintain.md`, and `docs/github_issues.md` to reflect Phase 11 cross-platform capabilities.

## [0.11.0-alpha] - 2026-08-17 — Phase 10: Interactive Physics & Input Mode

### Added
- Interaction domain model (`Interaction` enum in `src/core/interaction.rs`) supporting radial shockwaves, directional drag stirring, acoustic wave ripples, scroll pressure, and localized thermal pulses.
- Bounded interaction physics in `Simulation::apply_interaction()` preventing velocity explosion and thermal instability.
- Terminal-to-simulation coordinate mapper (`src/input/coords.rs`) converting character grid cells to normalized $[0.0, 1.0]$ simulation coordinates with accurate vertical inversion.
- Stateful `MouseTracker` (`src/input/mouse.rs`) converting mouse clicks to shockwaves, dragging to fluid stirring velocity vectors, right clicks to thermal pulses, and scroll events to buoyancy pressure changes.
- Keyboard typing acoustic wave ripples (`map_key_event_with_ripple` in `src/input/keyboard.rs`).
- Fail-safe terminal mouse capture lifecycle (`EnableMouseCapture`/`DisableMouseCapture`) with panic hook and signal handler cleanup.
- CLI switches: `--no-mouse`, `--no-ripple`, `--shockwave-force <FORCE>`, `--stir-force <FORCE>`.
- TOML configuration `[interaction]` schema supporting `mouse`, `keyboard_ripple`, `shockwave_force`, and `stir_force`.
- Comprehensive unit and integration test suite covering coordinate mapping, drag tracking, shockwave propagation, and full interactive pipeline.

### Fixed & Hardened (Phase 10 Review Findings)
- Enforced strict finite floating-point validation (`is_finite()`) across all configuration fields, rejecting `NaN`, `+Infinity`, and `-Infinity`.
- Aligned `shockwave_force` and `stir_force` validation with documentation to strictly enforce bounded `[0.1, 10.0]` range.
- Made panic hook dynamically aware of actual execution mode (`is_fullscreen`), preventing improper `LeaveAlternateScreen` invocations during inline execution.
- Filtered `KeyModifiers::CONTROL` (except `Ctrl+C`) and `KeyModifiers::ALT` in keyboard mapping to prevent terminal control shortcuts from becoming spurious ripple events.
- Isolated mouse drag tracking exclusively to left-button interactions, preventing right-click events from perturbing drag vectors.
- Normalized mouse drag stirring velocity by elapsed sampling time $\Delta t$ (`dx / dt`, `dy / dt`), eliminating terminal mouse-event rate dependency.
- Added stress and regression test suites covering rapid shockwave bursts, continuous drag sequences, compact scaling resets, modifier keys, and floating-point boundary conditions.

## [0.10.0] - 2026-08-16 — Phase 9: Multiplexer & Widget Mode (tmux / zellij)

### Added
- Multiplexer environment adapter (`src/widget/multiplexer.rs`) detecting active `tmux` (`TMUX`) and `zellij` (`ZELLIJ`) contexts with testable dependency injection abstractions.
- Adaptive compact geometry scaler (`src/widget/compact.rs`) with deterministic `CompactProfile` calculation adapting blob counts (2–8) and particle radii for micro/compact split-panes (`< 40` cols or `< 15` rows).
- Single-shot ANSI True Color frame serializer (`render_snapshot` in `src/widget/snapshot.rs`) for embedding into `tmux` status bars (`status-right`), `zellij` plugins, polybar, and external scripts.
- In-place interactive inline rendering mode (`--inline`) operating cleanly without alternate-screen allocation.
- Centralized widget policy resolver (`src/widget/policy.rs`) enforcing precedence (`CLI > TOML > Defaults`) and rejecting conflicting execution flags.
- CLI switches: `--fps <FPS>`, `--compact`, `--widget`, `--inline`, `--snapshot`, `--width <COLS>`, `--height <ROWS>`.
- TOML configuration `[widget]` schema supporting `compact`, `fps`, `inline`, `width`, `height`, and `adapt_blobs`.
- Comprehensive unit and integration test coverage across micro-geometries (`20x1`, `20x2`, `20x3`), compact scaling, and policy validation.

### Fixed & Hardened (v0.10.0 Stabilization)
- Fixed Arch Linux release packaging CI and local build scripts with native `PKGBUILD` and `makepkg` source tarball packaging.
- Applied `radius_scale` in `CompactScaler` and `Simulation`, scaling individual blob radii alongside physics parameters in compact mode.
- Removed `panic = "abort"` in release profile to ensure proper panic hooks and terminal state restoration.
- Added safe Unix signal handling for `SIGINT` and `SIGTERM`, ensuring raw mode is disabled and cursor restored on external termination.
- Validated target frame rate in `resolve_policy`, rejecting `fps == 0` with typed `LavaError::Config`.
- Fixed inline mode terminal resize logic to preserve configured/target height rather than hardcoding 10 rows.
- Converted `compute_fft` in `src/audio/fft.rs` from panicking `assert!` statements to returning typed `LavaError::Audio` validation errors.
- Enforced configuration validation on default config fallback paths in `load_config(None)`.
- Scoped internal Linux `/proc` and `/sys` parsers to `pub(crate)`.
- Added `$XDG_CONFIG_HOME` discovery support in `default_config_path()`.
- Added `thermal_transfer_rate` to `SimulationConfig` and wired into runtime physics initialization.
- Implemented `gradient: bool` support in `rasterize_simulation_options` and removed obsolete `double_buffering` field.
- Hardened CI and Release GitHub Actions workflows with pinned commit SHAs, restricted permissions, and `cargo-audit` step.
- Updated `SECURITY.md` supported version table to `0.10.x`.

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
