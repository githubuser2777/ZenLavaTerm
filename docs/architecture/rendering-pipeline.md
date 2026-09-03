# Rendering Pipeline & Terminal Encodings

The rendering subsystem converts the continuous 2D scalar potential field into an offscreen virtual framebuffer, which is then mapped to optimized ANSI escape sequences for terminal display.

---

## 1. Framebuffer Rasterization

```text
[Simulation Potential Field: F(x, y)]
                 │
                 ▼ (Discretization & Palette Mapping)
[Virtual Framebuffer: 2D Vec<Rgb>]
                 │
                 ▼ (Sub-cell Character Packing)
[Terminal Character Grid (Cols x Rows)]
                 │
                 ▼ (Double-Buffer Diffing & ANSI Batching)
[Stdout TTY Stream (Single write_all() call)]
```

### 1.1 Virtual Framebuffer (`src/render/framebuffer.rs`)
- Contiguous linear `Vec<Rgb>` storing dimensions $W_v \times H_v$.
- Direct slice indexing eliminates per-pixel bounds checks in inner rendering loops.
- Supports smooth linear interpolation (`lerp`) or precomputed stepped gradients for maximum throughput.

---

## 2. Terminal Character Encodings

Terminal cells are typically twice as tall as they are wide (aspect ratio $\approx 1:2$). ZenLavaTerm provides three renderers to optimize resolution and compatibility:

### 2.1 Half-Block Renderer (`halfblock`, Default)
- Uses Unicode character `▀` (Upper Half Block, `U+2580`).
- **Foreground Color**: Top virtual pixel $(x, 2y)$.
- **Background Color**: Bottom virtual pixel $(x, 2y + 1)$.
- Doubles the effective vertical resolution of the terminal ($W_v = \text{Cols}$, $H_v = 2 \times \text{Rows}$).

```text
Virtual Pixels:         Terminal Cell:
+-------------------+   +-------------------+
| Top Pixel (RGB)   |   | ▀ Foreground      |
+-------------------+ → +-------------------+
| Bottom Pixel (RGB)|   | ▄ Background      |
+-------------------+   +-------------------+
```

### 2.2 Full-Block Renderer (`block`)
- Uses Unicode full block `█` (`U+2588`) with 24-bit foreground color.
- Matches standard 1:1 terminal rows ($W_v = \text{Cols}$, $H_v = \text{Rows}$).
- Provides maximum compatibility with legacy or constrained terminal emulators.

### 2.3 Braille Matrix Renderer (`braille`)
- Uses Unicode Braille patterns (`U+2800`..`U+28FF`).
- Packs an 8-subpixel matrix ($2 \times 4$ dots) into every terminal character cell.
- Effective virtual resolution: $W_v = 2 \times \text{Cols}$, $H_v = 4 \times \text{Rows}$.
- Generates high-density contour outlines of fluid surfaces.

---

## 3. High-Performance Output Optimizations

1. **Batched `BufWriter`**:
   - Rather than executing `print!()` per character, entire frames are formatted into a contiguous memory buffer and flushed in a single `write_all()` call.
   - Eliminates terminal flickering and minimizes OS context switches.
2. **True-Color ANSI Minimization**:
   - Sequences format as `\x1b[38;2;r;g;bm` (foreground) and `\x1b[48;2;r;g;bm` (background).
   - Adjacent cells sharing identical colors omit redundant escape sequences, saving terminal bandwidth.
3. **Double Buffering & Dirty Diffing**:
   - Compares the newly rendered back buffer with the front buffer; unmodified cells can be skipped to avoid unnecessary terminal cursor repositions.
