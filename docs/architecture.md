# LavaTerm Architecture

## 1. Executive Summary

**LavaTerm** is designed with a strict unidirectional data flow, separating pure mathematical simulation from terminal rendering and OS-level platform signals. The core simulation runs independently of terminal dimensions, frame rate, or ANSI terminal sequences.

---

## 2. Unidirectional Data Pipeline

```text
+--------------------------------------------------------+
|                   Signals & Input                      |
| (Time delta, Keyboard events, future Audio/System)    |
+---------------------------+----------------------------+
                            |
                            v
+--------------------------------------------------------+
|                   Simulation Core                      |
| (Blobs, Physics, Buoyancy, Gravity, Scalar Field)      |
+---------------------------+----------------------------+
                            |
                            v
+--------------------------------------------------------+
|                 Virtual Framebuffer                    |
| (2D RGB Pixel Array, Color Palettes, Double Buffering) |
+---------------------------+----------------------------+
                            |
                            v
+--------------------------------------------------------+
|                  Terminal Renderer                     |
| (Half-block / Block / Braille conversion to ANSI)      |
+---------------------------+----------------------------+
                            |
                            v
+--------------------------------------------------------+
|                  Terminal Output / TTY                 |
| (Raw mode, alternate screen, batched stdout flush)     |
+--------------------------------------------------------+
```

---

## 3. Module Hierarchy & Dependency Direction

The crate is organized into four main layers:

```text
lavaterm (binary: src/main.rs)
   ├── input      (Event polling, keyboard action mapping)
   ├── config     (TOML parser, validation, defaults)
   ├── render     (Framebuffer, RGB math, Half-block/Block renderers)
   └── core       (Pure simulation, Blobs, Physics, Scalar Field)
```

### Strict Architectural Boundaries:
1. **`core` has zero terminal dependencies**:
   - `core` MUST NOT import `crossterm` or any platform-specific graphics/audio crates.
   - Simulation operates within normalized space (e.g. `[0.0, 1.0] x [0.0, 1.0]`) or continuous virtual coordinates.
2. **`render` depends only on data models and terminal primitives**:
   - The renderer consumes a read-only snapshot or borrow of `VirtualFramebuffer` and generates batched ANSI output.
   - `render` does NOT compute physics or alter simulation state.
3. **`config` is pure data**:
   - Config structs are plain data transfer objects (DTOs) with validation logic. They do not hold runtime handles.
4. **`input` translates raw events**:
   - Translates raw `crossterm::event` into high-level domain actions (`Action::Quit`, `Action::Pause`, `Action::Reset`).

---

## 4. Subsystems Detail

### 4.1. Core Simulation (`src/core/`)
- **`metaball.rs`**: Defines the `Blob` struct containing `position`, `velocity`, `radius`, and `temperature`.
- **`physics.rs`**: Computes forces:
  - Thermal Buoyancy (hotter blobs rise, colder sink)
  - Gravity ($g$)
  - Viscosity / drag damping ($v \leftarrow v \cdot (1 - \mu \cdot \Delta t)$)
  - Boundary elastic reflection and damping.
- **`field.rs`**: Evaluates the cumulative scalar field at any $(x, y)$ coordinate:
  $$F(x, y) = \sum_{i=1}^{N} \frac{R_i^2}{(x - x_i)^2 + (y - y_i)^2 + \epsilon}$$
- **`simulation.rs`**: Maintains the active blob collection and advances time using explicit $\Delta t$.

### 4.2. Virtual Framebuffer & Color (`src/render/`)
- **`framebuffer.rs`**: Holds a 2D grid `Vec<Rgb>` of size `width x height`.
- **`color.rs`**: Provides `Rgb` structs, linear interpolation (`lerp`), and multi-stop gradient color maps.
- **`halfblock.rs`**: Half-block renderer packing two vertical virtual pixels $(x, 2y)$ and $(x, 2y + 1)$ into a single character cell `▀` using:
  - Foreground color = top pixel RGB
  - Background color = bottom pixel RGB

### 4.3. Terminal Backend & Lifecycle (`src/main.rs`)
- Safely initializes raw mode and enters the alternate screen buffer via `crossterm`.
- Installs a custom panic hook ensuring that even if a panic occurs, the terminal cursor is shown and the alternate screen is exited cleanly.

---

## 5. Failure and Error Handling

- All public functions return `Result<T, LavaError>`.
- No uncontrolled panics or `.unwrap()` calls in the runtime loop.
- Graceful degradation: if a subsystem (such as audio or system metric provider) is unavailable, the core simulation continues with fallback defaults.
