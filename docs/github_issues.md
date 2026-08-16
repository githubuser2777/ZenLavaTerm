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
