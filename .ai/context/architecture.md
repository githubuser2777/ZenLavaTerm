# ZenLavaTerm Architecture Context for AI Agents

> **Note**: Authoritative human-facing architecture documentation is maintained in [docs/architecture/](file:///home/skids/Documents/code/ZenLavaTerm/docs/architecture/). This file provides a condensed context reference for AI coding agents.

---

## 1. System Pipeline & Unidirectional Data Flow

ZenLavaTerm operates on a strictly decoupled, unidirectional data pipeline:

```text
+─────────────────────────────────────────────────────────────+
│                       Platform Signals                      │
│  - System Telemetry (procfs / Win32 / Mach kernel)          │
│  - Audio Stream (CPAL PCM RingBuffer / Synthetic fallback)  │
│  - Terminal User Input (Keyboard commands, Mouse drag/click)│
+──────────────────────────────┬──────────────────────────────+
                               │ Normalizes into domain structs
                               ▼
+─────────────────────────────────────────────────────────────+
│                     Simulation Core                         │
│  - N Metaball Blobs (positions, velocities, temperatures)   │
│  - Classical Physics (Buoyancy, Gravity, Viscosity Drag)    │
│  - Interaction Impulses (Shockwave, Stir, Ripple, Pressure) │
│  - Continuous Scalar Potential Field Evaluation F(x, y)     │
+──────────────────────────────┬──────────────────────────────+
                               │ Discretizes into 2D RGB buffer
                               ▼
+─────────────────────────────────────────────────────────────+
│                   Virtual Framebuffer                       │
│  - Contiguous 2D Pixel Grid (width x height x Rgb)          │
│  - Color Palette Mapping (Smooth / Stepped Gradients)       │
│  - Double Buffering & Dirty Cell Diffing                    │
+──────────────────────────────┬──────────────────────────────+
                               │ Sub-cell character encoding
                               ▼
+─────────────────────────────────────────────────────────────+
│                    Terminal Renderer                        │
│  - Half-Block (▀ - 2x vertical resolution)                  │
│  - Full-Block (█ - standard 1:1 character cells)            │
│  - Braille (U+2800..U+28FF - 2x4 dot matrix subpixels)      │
+──────────────────────────────┬──────────────────────────────+
                               │ Batched Stdout Write
                               ▼
+─────────────────────────────────────────────────────────────+
│               Terminal Output Stream (TTY)                  │
│  - Batched BufWriter flushing in single write_all()         │
│  - ANSI True-Color escape sequences                         │
+─────────────────────────────────────────────────────────────+
```

---

## 2. Rust Crate Architecture & Module Breakdown

The crate root is [src/lib.rs](file:///home/skids/Documents/code/ZenLavaTerm/src/lib.rs) (`lavaterm`), and the binary entry point is [src/main.rs](file:///home/skids/Documents/code/ZenLavaTerm/src/main.rs):

| Module | Location | Primary Responsibility |
|---|---|---|
| **`core`** | [src/core/](file:///home/skids/Documents/code/ZenLavaTerm/src/core/) | Pure mathematical simulation: metaballs, scalar field, thermal physics, interactions. Zero dependencies on crossterm or OS. |
| **`audio`** | [src/audio/](file:///home/skids/Documents/code/ZenLavaTerm/src/audio/) | CPAL audio capture stream, SPSC lock-free Seqlock ring buffer, Radix-2 FFT spectrum analysis, synthetic fallback. |
| **`reactive`** | [src/reactive/](file:///home/skids/Documents/code/ZenLavaTerm/src/reactive/) | Cross-platform OS metrics providers (Linux procfs, Windows Win32, macOS Mach kernel). |
| **`render`** | [src/render/](file:///home/skids/Documents/code/ZenLavaTerm/src/render/) | Virtual framebuffer, RGB interpolation, halfblock, block, and braille ANSI renderers. |
| **`input`** | [src/input/](file:///home/skids/Documents/code/ZenLavaTerm/src/input/) | Crossterm event mapping, mouse drag vectors, keyboard wave ripples, coordinate inversion. |
| **`config`** | [src/config/](file:///home/skids/Documents/code/ZenLavaTerm/src/config/) | TOML configuration parser, default paths resolution, Serde validation and field aliases. |
| **`theme`** | [src/theme/](file:///home/skids/Documents/code/ZenLavaTerm/src/theme/) | Preset palettes, auto-detection, Pywal/Wallust JSON parsers, custom palette files. |
| **`widget`** | [src/widget/](file:///home/skids/Documents/code/ZenLavaTerm/src/widget/) | Multiplexer detection (tmux/zellij), compact geometry scaling, snapshot rendering, execution policies. |

---

## 3. Terminal-Native TUI Paradigm vs GUI/Tauri

- **Terminal-Native TUI**: ZenLavaTerm is built directly on terminal I/O using `crossterm`. It writes ANSI escape sequences to standard output in raw terminal mode.
- **Not a GUI / Not Tauri**: The project does **not** use Tauri, webviews, HTML/CSS, WebGL, or GUI windowing toolkits.
- **Multiplexer & Widget Integration**: The `widget` module provides adaptive profile scaling (`CompactProfile`, `CompactScaler`) and single-frame snapshot rendering (`--snapshot`) designed for terminal multiplexer status bars, tiling window managers, and scripts.

---

## 4. Concurrency & Threading Model

- **Main / Visualization Thread**: Runs the terminal event loop, polls inputs via `crossterm::event::poll`, steps the simulation $\Delta t$, rasterizes the framebuffer, and writes ANSI escape batches to `stdout`.
- **Audio Capture Thread**: Spawned by `cpal::traits::StreamTrait::play()`, ingests raw PCM frames into the lock-free `PcmRingBuffer`. Readers verify sequence integrity using the 64-bit Seqlock (`version: AtomicU64`).
