# LavaTerm Rendering Pipeline

## 1. Pipeline Overview

The rendering subsystem converts the continuous scalar field into a discrete 2D virtual pixel buffer, and subsequently translates that buffer into optimized ANSI escape sequences for high-performance terminal rendering.

```text
[Simulation Field: F(x, y)]
             │
             ▼ (Discretization & Palette Mapping)
[Virtual Framebuffer (Width x Height, RGB)]
             │
             ▼ (Sub-cell Character Packing)
[Terminal Cells (Width x (Height / 2))]
             │
             ▼ (Diffing & ANSI Batching)
[Stdout TTY Stream]
```

---

## 2. Virtual Canvas & Framebuffer

The `VirtualFramebuffer` acts as an offscreen 2D pixel array:
- Resolution: $W_v \times H_v$ (virtual width and height).
- For half-block mode, $W_v = \text{Terminal Columns}$ and $H_v = 2 \times \text{Terminal Rows}$.
- Each pixel stores an `Rgb { r: u8, g: u8, b: u8 }` value or field intensity.

---

## 3. Terminal Character Encodings

### 3.1. Half-Block Renderer (`halfblock`)

Terminal characters are typically twice as tall as they are wide (aspect ratio $\approx 1:2$). Using Unicode half-block characters effectively doubles the vertical resolution:

- Character `▀` (Upper Half Block, `U+2580`):
  - **Foreground ANSI color**: Color of virtual pixel $(x, 2y)$ (top).
  - **Background ANSI color**: Color of virtual pixel $(x, 2y + 1)$ (bottom).

```text
Virtual Pixels:       Terminal Cell:
+-----------------+   +-----------------+
| Top Pixel (RGB) |   | ▀ Foreground    |
+-----------------+ → +-----------------+
| Btm Pixel (RGB) |   | ▄ Background    |
+-----------------+   +-----------------+
```

### 3.2. Full-Block Renderer (`block`)

Standard fallback using the full block character `█` (`U+2588`) with 24-bit foreground color:
- Virtual height equals terminal rows ($H_v = \text{Terminal Rows}$).

---

## 4. True-Color ANSI Encoding

Modern terminal emulators support 24-bit True Color (DirectColor) via standard SGR escape sequences:

- **Foreground**: `\x1b[38;2;<r>;<g>;<b>m`
- **Background**: `\x1b[48;2;<r>;<g>;<b>m`
- **Reset**: `\x1b[0m`

LavaTerm formats color changes concisely, avoiding redundant color escapes when adjacent cells share identical top and bottom colors.

---

## 5. Buffering & Output Optimization

### 5.1. Double Buffering & Dirty-Cell Tracking
- LavaTerm maintains a `front_buffer` (currently on screen) and `back_buffer` (newly rendered).
- During rendering, cells that have not changed between frames are skipped, minimizing terminal cursor repositioning and bandwidth.

### 5.2. Batched `BufWriter`
- Never invoke `print!()` or `write!()` per pixel.
- All ANSI escape sequences for a full frame are written to an in-memory byte buffer (`Vec<u8>` or `std::io::BufWriter`) and flushed to `std::io::stdout()` in a single `write_all()` syscall.
- Result: 60 FPS animation with zero tearing or flickering.
