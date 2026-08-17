# LavaTerm GitHub Milestone Issues Specification

This document defines the initial set of structured GitHub issues for the LavaTerm project roadmap.

---

### Issue 01: Project Bootstrap & Skeleton
- **Goal**: Initialize the Rust project repository with production conventions, CI, and buildable module skeletons.
- **Context**: Establish solid foundation before implementing mathematical simulation or terminal rendering.
- **Scope**: `Cargo.toml`, `.gitignore`, `LICENSE`, `README.md`, `rustfmt.toml`, `.github/workflows/ci.yml`, and module layouts.
- **Non-goals**: Implementing full terminal event loop or complex physics.
- **Acceptance Criteria**:
  - `cargo build` and `cargo test` pass cleanly.
  - `cargo fmt --check` and `cargo clippy -- -D warnings` pass.
  - Minimal headless CLI runner executable.
- **Technical Notes**: Decouple `core` from `crossterm`.
- **Dependencies**: None.

---

### Issue 02: Core Data Model & Blob Representation
- **Goal**: Implement the `Blob` data structure and unit tests for particle state representation.
- **Context**: Foundation for metaball calculations and thermodynamic convection.
- **Scope**: `src/core/metaball.rs`.
- **Non-goals**: Terminal rendering, memory pooling.
- **Acceptance Criteria**:
  - `Blob` struct with `position (x, y)`, `velocity (vx, vy)`, `radius`, `temperature`.
  - Constructors with validation (e.g. non-negative radius).
  - 100% unit test coverage for state initialization.
- **Technical Notes**: Use `f32` coordinates normalized in $[0.0, 1.0]$.
- **Dependencies**: Issue 01.

---

### Issue 03: Metaball Scalar Field Evaluation
- **Goal**: Implement 2D scalar potential evaluation from active blobs.
- **Context**: Computes fluid density at any coordinate point.
- **Scope**: `src/core/field.rs`.
- **Non-goals**: Color gradients or terminal characters.
- **Acceptance Criteria**:
  - `ScalarField` struct evaluating $F(x, y) = \sum f(d_i)$.
  - Smooth polynomial falloff or inverse-square kernel.
  - Deterministic unit tests proving field decreases with distance and superimposes for close blobs.
- **Technical Notes**: Avoid division by zero via epsilon constant.
- **Dependencies**: Issue 02.

---

### Issue 04: Basic Fluid Physics & Convection
- **Goal**: Implement buoyancy, gravity, and viscous drag numerical integration for blobs.
- **Context**: Gives the blobs natural lava lamp movement.
- **Scope**: `src/core/physics.rs` and `src/core/simulation.rs`.
- **Non-goals**: Particle collisions or Navier-Stokes simulation.
- **Acceptance Criteria**:
  - Thermal buoyancy accelerates hot blobs upwards and cold blobs downwards.
  - Viscous drag dampens velocities over time.
  - Numerical integration bounded by $\Delta t_{\text{max}}$.
  - Deterministic unit tests for motion step integration.
- **Technical Notes**: Euler-Cromer integration with explicit $\Delta t$.
- **Dependencies**: Issue 03.

---

### Issue 05: Virtual Framebuffer Abstraction
- **Goal**: Implement an in-memory 2D RGB grid buffer decoupled from terminal resolution.
- **Context**: Serves as the intermediate canvas between simulation and terminal rendering.
- **Scope**: `src/render/framebuffer.rs`.
- **Non-goals**: Direct terminal syscalls.
- **Acceptance Criteria**:
  - `VirtualFramebuffer` struct with `width`, `height`, and pixel accessor methods.
  - Bounds-checked pixel reading and writing.
  - Double buffer support for dirty region diffing.
- **Technical Notes**: Contiguous 1D vector `Vec<Rgb>` for cache locality.
- **Dependencies**: Issue 01.

---

### Issue 06: True-Color Gradient & Palette Mapping
- **Goal**: Implement linear color interpolation (`lerp`) and multi-stop gradient mapping from field values to RGB.
- **Context**: Visual aesthetic core of the lava lamp fluid.
- **Scope**: `src/render/color.rs`.
- **Non-goals**: Dynamic theme auto-detection (Phase 8).
- **Acceptance Criteria**:
  - `Rgb` color struct with hex parsing (`#rrggbb`) and linear interpolation.
  - `ColorPalette` struct mapping normalized scalars $[0.0, 1.0]$ to gradient RGB values.
  - Unit tests for color blending and hex parsing edge cases.
- **Technical Notes**: Clamping inputs to $[0.0, 1.0]$ before interpolating.
- **Dependencies**: Issue 05.

---

### Issue 07: Half-Block Unicode Terminal Renderer
- **Goal**: Implement high-resolution terminal rendering by packing two vertical virtual pixels per character cell (`▀`).
- **Context**: Doubles vertical resolution in modern terminal emulators.
- **Scope**: `src/render/halfblock.rs`.
- **Non-goals**: Braille rendering (Phase 4).
- **Acceptance Criteria**:
  - Converts top pixel to ANSI 24-bit foreground and bottom pixel to background.
  - Batched output writing into `BufWriter` for flicker-free rendering.
  - Visual verification with sample gradient buffers.
- **Technical Notes**: SGR escape sequences `\x1b[38;2;r;g;bm` and `\x1b[48;2;r;g;bm`.
- **Dependencies**: Issue 05, Issue 06.

---

### Issue 08: Main Event & Render Loop
- **Goal**: Assemble simulation updates, framebuffer rasterization, and terminal flushing into a 30/60 FPS loop.
- **Context**: Delivers the interactive ambient visualizer binary.
- **Scope**: `src/main.rs`.
- **Non-goals**: Audio analysis, widget mode.
- **Acceptance Criteria**:
  - Target framerate timing via delta time measurement.
  - Non-blocking keyboard event handling (`q` / `Esc` to exit).
  - Safe terminal initialization and panic hook restoration.
- **Technical Notes**: Use `crossterm::event::poll`.
- **Dependencies**: Issue 04, Issue 07.

---

### Issue 09: TOML Configuration Engine
- **Goal**: Support custom TOML configuration files and CLI overrides.
- **Context**: Allows users to customize blob count, physics parameters, and color palettes.
- **Scope**: `src/config/mod.rs` and `src/config/schema.rs`.
- **Non-goals**: Remote config syncing.
- **Acceptance Criteria**:
  - Serde deserialization for `[simulation]`, `[render]`, `[palette]`.
  - CLI argument `--config <path>` loading.
  - Graceful validation error reporting on malformed config.
- **Technical Notes**: Sensible default fallback when config is omitted.
- **Dependencies**: Issue 01.

---

### Issue 10: Dynamic Terminal Resize Handling
- **Goal**: Adaptively resize virtual framebuffer and recompute aspect ratios when terminal window resizes.
- **Context**: Prevents visual distortion or out-of-bounds rendering on terminal window resize.
- **Scope**: `src/render/mod.rs` and `src/main.rs`.
- **Non-goals**: Complex multi-window layouts.
- **Acceptance Criteria**:
  - Listens for `crossterm::event::Event::Resize(cols, rows)`.
  - Reallocates or resizes `VirtualFramebuffer` without crashing or artifacting.
- **Technical Notes**: Clear screen and invalidate double-buffer cache on resize.
- **Dependencies**: Issue 08.

---

### Issue 11: Performance Benchmark Suite
- **Goal**: Create micro-benchmarks for scalar field evaluation and half-block ANSI generation.
- **Context**: Ensure smooth 60 FPS performance on modest hardware and large terminal dimensions.
- **Scope**: `benches/` or headless performance metrics.
- **Non-goals**: Premature micro-optimizations before baseline.
- **Acceptance Criteria**:
  - Benchmark measuring field calculation time per cell across various blob counts (6, 12, 24).
  - Benchmark measuring ANSI serialization throughput.
- **Technical Notes**: Criterion.rs or standard timing harness.
- **Dependencies**: Issue 07, Issue 08.

---

### Issue 12: Developer & User Documentation
- **Goal**: Produce complete, structured architecture, configuration, and simulation documentation.
- **Context**: Enable seamless developer onboarding and user configuration.
- **Scope**: `docs/` and `README.md`.
- **Non-goals**: Video tutorials.
- **Acceptance Criteria**:
  - All architecture, rendering, simulation, and configuration docs finalized.
  - Clear quick start and troubleshooting guide in README.
- **Technical Notes**: Markdown with GFM standards.
- **Dependencies**: Issue 01.

---

### Issue 13: CI/CD Pipeline Automation
- **Goal**: Automated quality gate enforcement across Linux, macOS, and Windows.
- **Context**: Guard against formatting errors, lint violations, and broken builds.
- **Scope**: `.github/workflows/ci.yml`.
- **Non-goals**: Auto-publishing releases to package registries (Phase 12).
- **Acceptance Criteria**:
  - GitHub Actions runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build`.
  - Cross-platform matrix test verification.
- **Technical Notes**: Cache cargo dependencies for fast turnaround.
- **Dependencies**: Issue 01.

---

### Issue 14: System Signal Abstraction & Mock Provider
- **Goal**: Define decoupled reactive system signal domain types and provider traits.
- **Context**: Enables simulation to react to OS metrics without tying core logic to OS APIs.
- **Scope**: `src/reactive/mod.rs` and `src/reactive/signals.rs`.
- **Non-goals**: Direct platform syscalls in core.
- **Acceptance Criteria**:
  - `SystemSignals` struct normalized in $[0.0, 1.0]$ (`cpu_load`, `memory_usage`, `battery_level`, `io_activity`).
  - `SystemProvider` trait for fetching signals with graceful error recovery.
  - `MockSystemProvider` for deterministic testing.
- **Dependencies**: Issue 04.

---

### Issue 15: Linux /proc and sysfs System Metrics Provider
- **Goal**: Implement zero-dependency native Linux metric extraction via `/proc/stat`, `/proc/meminfo`, and `/sys/class/power_supply`.
- **Context**: High-performance ambient visualizer integration on primary target platform (Linux).
- **Scope**: `src/platform/linux.rs` (or `src/reactive/linux.rs`).
- **Non-goals**: Heavy third-party daemon dependencies.
- **Acceptance Criteria**:
  - Reads CPU utilization, memory pressure, and battery status safely.
  - Returns `None` or default fallback if virtual files are unreadable (never panics).
  - Unit tests parsing mock procfs strings.
- **Dependencies**: Issue 14.

---

### Issue 16: Simulation Signal Mapping & Fluid Modulation
- **Goal**: Map normalized system signals to metaball physical properties (turbulence, size, convection rate).
- **Context**: CPU usage drives turbulence, RAM drives blob radius/count, battery drives thermal buoyancy.
- **Scope**: `src/core/simulation.rs` and `src/core/physics.rs`.
- **Non-goals**: Audio FFT spectrum.
- **Acceptance Criteria**:
  - Simulation smoothly modulates physics constants in response to signal updates.
  - Stable numerical integration even under sudden metric spikes.
  - Unit tests verifying parameter modulation.
- **Dependencies**: Issue 14, Issue 15.

---

### Issue 17: CLI & Configuration Integration for System Reactive Mode
- **Goal**: Add `--system` CLI switch and `[reactive]` TOML configuration options.
- **Context**: Allows users to enable ambient system monitoring mode with custom mappings.
- **Scope**: `src/config/schema.rs` and `src/main.rs`.
- **Non-goals**: External desktop widget embedding (Phase 9).
- **Acceptance Criteria**:
  - CLI flag `--system` enables real-time background metric polling.
  - Graceful degradation if OS metrics cannot be gathered.
  - Integration test verifying headless simulation with active system signals.
- **Dependencies**: Issue 15, Issue 16.

---

## Phase 7: Audio Reactive (v0.8.0)

### Issue 18: Audio Signal Abstraction & FFT Spectrum Analyzer
- **Goal**: Implement high-performance zero-dependency FFT and spectrum energy band binning.
- **Context**: Converts raw audio PCM samples into normalized frequency bands (`bass`, `mid`, `treble`).
- **Scope**: `src/audio/mod.rs`, `src/audio/fft.rs`, `src/audio/signals.rs`.
- **Non-goals**: Low-level audio hardware drivers in the FFT module.
- **Acceptance Criteria**:
  - Radix-2 / Cooley-Tukey FFT implementation with Hann windowing.
  - Spectrum band energy extractor mapping frequencies to `bass` (20-250Hz), `mid` (250-4000Hz), and `treble` (4000-20000Hz).
  - Deterministic unit tests with synthetic sine waves verifying frequency bin isolation.
- **Dependencies**: None.

---

### Issue 19: Audio Provider Trait & Mock / Synthetic Audio Provider
- **Goal**: Define `AudioProvider` trait and deterministic fixtures for testing without physical audio devices.
- **Context**: Enables audio reactivity simulation and testability on CI runners.
- **Scope**: `src/audio/provider.rs`.
- **Non-goals**: Physical ALSA/PipeWire device acquisition.
- **Acceptance Criteria**:
  - `AudioProvider` trait returning `AudioSignals`.
  - `MockAudioProvider` and `SyntheticAudioGenerator` (sine wave, pulse, sweep).
  - Unit tests verifying provider contract.
- **Dependencies**: Issue 18.

---

### Issue 20: Native Linux Audio Capture Integration & Ring Buffer
- **Goal**: Implement non-blocking PCM stream receiver with lockless ring-buffer for Linux.
- **Context**: Captures real audio playback from PipeWire or default audio stream without stalling the 60 FPS render loop.
- **Scope**: `src/audio/capture.rs` / `src/audio/linux.rs`.
- **Non-goals**: Modifying system sound server configuration.
- **Acceptance Criteria**:
  - Non-blocking ring buffer decoupling audio capture thread from render loop.
  - Graceful fallback to silence/mock provider if no audio device is present.
  - Unit tests for circular ring buffer overflow and read semantics.
- **Dependencies**: Issue 18, Issue 19.

---

### Issue 21: Simulation Audio Modulation & CLI `--audio` Integration
- **Goal**: Map audio spectrum signals to fluid metaball motion and add CLI/TOML controls.
- **Context**: Bass pulses buoyancy/velocity, mid-range modulates turbulence, treble vibrates surfaces.
- **Scope**: `src/core/simulation.rs`, `src/config/schema.rs`, `src/main.rs`.
- **Non-goals**: Full DAW plugin features.
- **Acceptance Criteria**:
  - `Simulation::apply_audio_signals()` smoothly modulates physics parameters.
  - CLI switch `--audio` and `[audio]` section in TOML config.
  - Integration test verifying audio-reactive simulation pipeline.
- **Dependencies**: Issue 18, Issue 19, Issue 20.

---

## Phase 8: Theme Engine (v0.9.0)

### Issue 22: Theme Domain Abstraction, Preset Palettes & Theme Provider Trait
- **Goal**: Define the core theme domain model, curated preset palettes, and the `ThemeProvider` trait.
- **Context**: Enables decoupling of visual theme resolution from terminal rendering and physics.
- **Scope**: `src/theme/mod.rs`, `src/theme/preset.rs`, `src/theme/provider.rs`.
- **Non-goals**: Hardcoding third-party editor themes into the physics core.
- **Acceptance Criteria**:
  - `Theme` struct and `ThemeProvider` trait returning `ColorPalette`.
  - Built-in curated presets: `lava` (default), `ocean`, `cyberpunk`, `synthwave`, `nord`, `forest`, `monochrome`, `matrix`.
  - Unit tests verifying color palette generation from presets and custom theme definitions.
- **Dependencies**: None.

---

### Issue 23: Pywal & Wallust Color Scheme Extractors
- **Goal**: Implement zero-dependency parsers for pywal and wallust cached color schemes.
- **Context**: Integrates LavaTerm into dynamic Linux desktop ricing environments (wal / wallust).
- **Scope**: `src/theme/pywal.rs`, `src/theme/wallust.rs`.
- **Non-goals**: Spawning external terminal commands or modifying X11/Wayland wallpapers.
- **Acceptance Criteria**:
  - `PywalExtractor` reading `~/.cache/wal/colors.json` and raw flat `colors` cache.
  - `WallustExtractor` reading `~/.cache/wallust/colors.json` or nix-colors.
  - Graceful fallback: returns standard fallback palette if cache files are missing or malformed (no panics).
  - Unit tests with mock JSON schemas and flat color string fixtures.
- **Dependencies**: Issue 22.

---

### Issue 24: Auto-Detection & Custom File Theme Engine
- **Goal**: Implement auto-detection orchestrator and arbitrary external file theme loader (`.json` / `.toml`).
- **Context**: Enables `--theme auto` to automatically find active desktop colors and `--theme <path>` for custom user schemes.
- **Scope**: `src/theme/detector.rs`, `src/theme/file.rs`.
- **Non-goals**: Polling file changes at 1000 Hz.
- **Acceptance Criteria**:
  - `AutoThemeProvider` checks Pywal, Wallust, and falls back to default preset.
  - `FileThemeProvider` reads and parses custom user theme JSON/TOML files.
  - Deterministic unit tests with mock filesystem paths and custom palette files.
- **Dependencies**: Issue 22, Issue 23.

---

### Issue 25: CLI `--theme`, TOML `[theme]` Configuration & Event Loop Integration
- **Goal**: Add CLI flag `--theme <name|auto|path>`, TOML `[theme]` configuration section, and integrate theme resolution in `main.rs`.
- **Context**: Users can choose themes via CLI or config file, and see dynamic colors in interactive and headless modes.
- **Scope**: `src/config/schema.rs`, `src/main.rs`, `tests/integration_test.rs`.
- **Non-goals**: Runtime in-app interactive GUI theme editor.
- **Acceptance Criteria**:
  - CLI flag `--theme` accepts presets (`ocean`, `cyberpunk`, etc.), `auto`, `pywal`, `wallust`, or file paths.
  - TOML `[theme]` config section supports `name = "..."`, `path = "..."`.
  - Integration tests verifying all theme modes in end-to-end rendering pipeline.
- **Dependencies**: Issue 22, Issue 23, Issue 24.

---

## Phase 9: Multiplexer & Widget Mode (v0.10.0)

### Issue 26: Multiplexer Environment Adapter
- **Goal**: Implement zero-dependency environment detection for tmux and zellij terminal multiplexers.
- **Context**: Enables LavaTerm to adapt its behavior when executed inside multiplexer panes and status bars.
- **Scope**: `src/widget/mod.rs`, `src/widget/multiplexer.rs`.
- **Non-goals**: Rendering logic or altering terminal buffers inside detection module.
- **Acceptance Criteria**:
  - `MultiplexerKind` enum (`Tmux`, `Zellij`, `GenericTerminal`).
  - Decoupled `detect_multiplexer_with` getter for deterministic testing without mutating global environment.
  - Unit tests covering `TMUX`, `ZELLIJ`, and generic terminal detection.
- **Dependencies**: None.

---

### Issue 27: Adaptive Geometry & Compact Profiles
- **Goal**: Implement deterministic geometry-based compact activation and physics profile scaling for small viewports.
- **Context**: Prevents particle saturation in micro and compact split-panes (e.g. 20x8 or status bars).
- **Scope**: `src/widget/compact.rs`.
- **Non-goals**: Modifying the simulation core or branching physics inside core algorithms.
- **Acceptance Criteria**:
  - `should_compact(cols, rows, explicit_flag)` determines activation cleanly.
  - `CompactScaler::calculate_profile` maps viewport dimensions into `CompactProfile` (`blob_count`, `radius_scale`, `buoyancy_scale`, `noise_scale`).
  - Unit tests verifying profile scaling across `10x3`, `15x5`, `20x8`, `24x8`, `40x15`, `80x24`, `200x60`.
- **Dependencies**: Issue 26.

---

### Issue 28: Widget Output Modes (Snapshot & Inline)
- **Goal**: Implement single-shot ANSI True Color frame serialization and in-place inline rendering.
- **Context**: Enables status bar embeddings (tmux `status-right`, polybar) and inline terminal usage without stealing alternate screen.
- **Scope**: `src/widget/snapshot.rs`, `src/render/mod.rs`.
- **Non-goals**: Interactive keyboard handling in snapshot mode.
- **Acceptance Criteria**:
  - `render_snapshot` returns an in-memory ANSI True Color `String` with zero terminal state changes.
  - Micro-geometry support (`20x1`, `20x2`, `20x3`, `24x8`, `80x24`) across `halfblock`, `block`, and `braille`.
  - Unit tests directly asserting on serialized string structure and ANSI SGR sequences.
- **Dependencies**: Issue 26, Issue 27.

---

### Issue 29: Policy Layer, CLI Integration, Configuration, Signals & Full Test Suite
- **Goal**: Wire up CLI flags, TOML `[widget]` configuration, execution policy resolution, cross-platform signal safety, and integration tests.
- **Context**: Complete end-to-end integration for the Phase 9 release (v0.10.0).
- **Scope**: `src/widget/policy.rs`, `src/config/schema.rs`, `src/main.rs`, `tests/integration_test.rs`.
- **Non-goals**: Mouse click shockwave interaction (Phase 10).
- **Acceptance Criteria**:
  - CLI flags `--fps`, `--compact`, `--widget`, `--inline`, `--snapshot`, `--width`, `--height`.
  - TOML `[widget]` schema supporting `compact`, `fps`, `inline`, `width`, `height`, `adapt_blobs`.
  - Policy resolver enforcing precedence and rejecting conflicting modes (`--snapshot` + `--inline`).
  - Cross-platform terminal state restoration on normal exit, panic, and signals.
  - End-to-end integration tests passing and all quality gates clean.
- **Dependencies**: Issue 26, Issue 27, Issue 28.

---

## Phase 10: Interactive Physics & Input Mode (v0.11.0)

### Issue 30: Interaction Domain Model & Simulation Physics
- **Goal**: Implement high-level interaction domain representation and fluid kinetics in `src/core/`.
- **Context**: Enables real-time kinetic and thermal modulation of metaballs (shockwaves, fluid stirring, ripples, pressure).
- **Scope**: `src/core/interaction.rs`, `src/core/simulation.rs`, `src/core/mod.rs`.
- **Non-goals**: Terminal event parsing in core logic.
- **Acceptance Criteria**:
  - `Interaction` enum (`Shockwave`, `Stir`, `Ripple`, `Pressure`, `ThermalPulse`).
  - `apply_shockwave`, `apply_stir`, `apply_ripple`, `apply_pressure`, and `apply_thermal_pulse` with bounded velocities and thermal transfer.
  - Deterministic unit tests proving radial shockwave falloff, stirring momentum transfer, and buoyancy pressure adjustments.
- **Dependencies**: None.

---

### Issue 31: Terminal Input Translation & Coordinate Normalizer
- **Goal**: Implement terminal-to-simulation coordinate mapping and stateful mouse/keyboard event translation.
- **Context**: Converts discrete terminal grid events (`crossterm::event::MouseEvent` and `KeyEvent`) into continuous domain interactions in $[0.0, 1.0]$.
- **Scope**: `src/input/coords.rs`, `src/input/mouse.rs`, `src/input/keyboard.rs`, `src/input/mod.rs`.
- **Non-goals**: Direct terminal I/O inside translation components.
- **Acceptance Criteria**:
  - `terminal_to_sim_coords(col, row, cols, rows)` maps cells accurately with vertical inversion (top row $\to$ sim cooling zone).
  - `MouseTracker` handles left clicks (shockwave), right clicks (thermal pulse), drag motion (stir velocity), and wheel scroll (pressure).
  - `map_key_event_with_ripple` translates character keypresses into wave ripple actions.
  - Unit tests verifying drag vector calculation and coordinate edge cases.
- **Dependencies**: Issue 30.

---

### Issue 32: Terminal Mouse Capture Lifecycle, Panic Hook & Signal Safety
- **Goal**: Implement safe initialization and guaranteed restoration of terminal mouse capture mode.
- **Context**: Prevents mouse capture escaping and hanging the user's terminal after application exit or panic.
- **Scope**: `src/main.rs`.
- **Non-goals**: Altering headless or snapshot output modes.
- **Acceptance Criteria**:
  - `EnableMouseCapture` and `DisableMouseCapture` paired safely.
  - Panic hook and Unix signal handlers (`SIGINT`/`SIGTERM`) reliably execute `DisableMouseCapture`.
  - Normal loop termination cleanly restores raw mode and cursor.
- **Dependencies**: Issue 31.

---

### Issue 33: Configuration `[interaction]`, CLI Integration & Policy Flags
- **Goal**: Add `[interaction]` TOML configuration section and CLI interaction flags.
- **Context**: Allows users to customize or disable mouse interaction, toggle keyboard ripples, and tune force multipliers.
- **Scope**: `src/config/schema.rs`, `src/main.rs`.
- **Non-goals**: In-app GUI settings menu.
- **Acceptance Criteria**:
  - TOML `[interaction]` config supporting `mouse`, `keyboard_ripple`, `shockwave_force`, `stir_force`.
  - CLI flags `--no-mouse`, `--no-ripple`, `--shockwave-force`, `--stir-force`.
  - Validation enforcing positive non-zero force multipliers.
- **Dependencies**: Issue 30, Issue 31, Issue 32.

---

### Issue 34: Phase 10 Comprehensive Test Suite & Documentation
- **Goal**: Implement end-to-end integration tests and update all documentation for v0.11.0.
- **Context**: Ensures regression-free release and complete developer/user onboarding.
- **Scope**: `tests/integration_test.rs`, `docs/architecture.md`, `docs/configuration.md`, `docs/roadmap.md`, `README.md`, `CHANGELOG.md`, `Cargo.toml`.
- **Non-goals**: Windows WASAPI capture (Phase 11).
- **Acceptance Criteria**:
  - End-to-end integration test verifying full interactive event mapping, simulation modulation, and rasterization pipeline.
  - All documentation updated with interactive controls and configuration reference.
  - All quality gates pass (`cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo build`).
- **Dependencies**: Issue 30, Issue 31, Issue 32, Issue 33.


