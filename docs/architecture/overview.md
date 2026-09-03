# Architecture Overview & Unidirectional Pipeline

ZenLavaTerm (`lavaterm`) is engineered around a strict unidirectional data pipeline that completely isolates mathematical fluid simulation from terminal escape sequence formatting and operating system telemetry.

---

## 1. Unidirectional Data Pipeline

```text
+-------------------------------------------------------------+
│                      Platform & Inputs                      │
│ - OS Telemetry (Linux procfs, Windows Win32, macOS Mach)   │
│ - Audio Streams (CPAL capture / Synthetic beat generator)   │
│ - TTY Inputs (crossterm keyboard actions, mouse drag vectors)│
+------------------------------+------------------------------+
                               │ Normalizes to [0.0, 1.0]
                               ▼
+-------------------------------------------------------------+
│                      Simulation Core                        │
│ - Physics State (N metaball blobs: position, velocity, temp)│
│ - Forces (Buoyancy, Gravity, Viscous Drag, Boundary Bounce) │
│ - Scalar Potential Field Evaluation F(x, y)                 │
+------------------------------+------------------------------+
                               │ Evaluates continuous field
                               ▼
+-------------------------------------------------------------+
│                    Virtual Framebuffer                      │
│ - Offscreen 2D RGB Pixel Array                              │
│ - Color Palette Gradient Mapping (Stepped or Smooth)        │
│ - Double Buffering & Dirty Cell Tracking                    │
+------------------------------+------------------------------+
                               │ Encodes character cells
                               ▼
+-------------------------------------------------------------+
│                     Terminal Renderer                       │
│ - Half-Block (▀ - 2 vertical subpixels per cell)            │
│ - Full-Block (█ - standard 1:1 cell)                        │
│ - Braille (U+2800..U+28FF - 2x4 subpixel dot matrix)        │
+------------------------------+------------------------------+
                               │ Single write_all() flush
                               ▼
+-------------------------------------------------------------+
│                   Terminal Stdout / TTY                     │
│ - ANSI 24-bit True-Color escape stream                      │
+-------------------------------------------------------------+
```

---

## 2. Module Boundaries & Dependency Invariants

The crate is organized into decoupled modules under `src/`:

```text
lavaterm (src/lib.rs, src/main.rs)
   ├── core       # Pure simulation, metaballs, physics, scalar field
   ├── audio      # Lock-free PCM ring buffer, FFT spectrum, capture
   ├── reactive   # Cross-platform system telemetry providers
   ├── render     # Virtual framebuffer, color maps, ANSI renderers
   ├── input      # Crossterm event translation, coordinate mapping
   ├── config     # TOML configuration schema, path discovery
   ├── theme      # Palette presets, Pywal/Wallust parsers, auto-detect
   └── widget     # Multiplexer detection, compact scaler, snapshot mode
```

### Strict Architectural Boundaries:
1. **`core` has ZERO platform or terminal dependencies**:
   - `core` operates strictly in continuous normalized coordinates (`[0.0, 1.0]`).
   - `core` never imports `crossterm`, `std::io::stdout`, or platform-specific headers.
2. **`reactive` and `audio` translate OS data into normalized domain signals**:
   - Telemetry providers expose the `SystemProvider` trait producing normalized `SystemSignals`.
   - Audio capture produces normalized `AudioSignals` (`bass`, `mid`, `treble` in `[0.0, 1.0]`).
   - All capture backends fail gracefully to deterministic synthetic generators when hardware is unavailable.
3. **`render` is a read-only consumer of `VirtualFramebuffer`**:
   - Renderers consume a read-only borrow of the framebuffer and generate ANSI escape sequences.
   - Renderers do not compute physics or alter simulation state.
4. **`input` normalizes terminal events**:
   - Maps discrete terminal rows/columns to continuous `[0.0, 1.0]` coordinates with vertical inversion.

---

## 3. Concurrency & Threading Model

- **Visualization Loop (Main Thread)**:
  - Polls terminal input with a frame-budget timeout via `crossterm::event::poll`.
  - Advances simulation by $\Delta t$.
  - Samples active telemetry and audio signals.
  - Rasterizes field into `VirtualFramebuffer`.
  - Emits ANSI sequences via `std::io::BufWriter` to `stdout`.
- **Audio Capture Thread (CPAL Worker)**:
  - Runs in the background under CPAL control.
  - Ingests PCM frames into the lock-free `PcmRingBuffer`.
  - Does not block or lock the visualization thread.
