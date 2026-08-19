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

The crate is organized into modular decoupled layers:

```text
lavaterm (binary: src/main.rs)
   ├── input      (Event polling, keyboard action mapping, mouse tracking)
   ├── config     (TOML parser, validation, defaults, cross-platform path resolution)
   ├── theme      (Presets, Pywal, Wallust, Auto-detection, Custom theme files)
   ├── audio      (FFT Spectrum Analyzer, PCM capture, synthetic audio generator)
   ├── reactive   (System metrics collector: Linux procfs/sysfs, Windows Win32 API, macOS Mach kernel, normalized signals)
   ├── render     (Framebuffer, RGB math, Half-block/Block/Braille renderers)
   ├── widget     (Multiplexer detection, compact scaler, snapshot serializer, policy engine)
   └── core       (Pure simulation, Blobs, Physics, Scalar Field, Interactions)
```

### Strict Architectural Boundaries:
1. **`core` has zero platform or terminal dependencies**:
   - `core` MUST NOT import `crossterm`, OS-specific APIs, or hardware crates.
   - Simulation operates strictly within normalized space (`[0.0, 1.0] x [0.0, 1.0]`) or continuous virtual coordinates.
2. **`reactive` and `audio` translate OS telemetry into normalized signals**:
   - Platform providers (`LinuxSystemProvider`, `WindowsSystemProvider`, `MacOSSystemProvider`, `MockSystemProvider`) expose the `SystemProvider` trait producing normalized `SystemSignals`.
   - Simulation consumes domain signals without knowing the host operating system.
3. **`render` depends only on data models and terminal primitives**:
   - The renderer consumes a read-only snapshot or borrow of `VirtualFramebuffer` and generates batched ANSI output.
   - `render` does NOT compute physics or alter simulation state.
4. **`config` is pure data with cross-platform discovery**:
   - Config structs are plain data transfer objects (DTOs) with validation logic.
   - Path discovery resolves standard configuration locations across Linux (`$XDG_CONFIG_HOME`, `~/.config`), Windows (`%APPDATA%`, `%USERPROFILE%`), and macOS (`~/Library/Application Support`, `~/.config`).
5. **`input` translates raw events**:
   - Translates raw `crossterm::event` into high-level domain actions (`Action::Quit`, `Action::Pause`, `Action::Reset`) and interactions (`Interaction::Shockwave`, `Interaction::Stir`, `Interaction::Pressure`, `Interaction::Ripple`).

---

## 4. Subsystems Detail

### 4.1. Core Simulation (`src/core/`)
- **`metaball.rs`**: Defines the `Blob` struct containing `position`, `velocity`, `radius`, and `temperature`.
- **`physics.rs`**: Computes forces:
  - Thermal Buoyancy (hotter blobs rise, colder sink)
  - Gravity ($g$)
  - Viscosity / drag damping ($v \leftarrow v \cdot (1 - \mu \cdot \Delta t)$)
  - Boundary elastic reflection and damping.
- **`interaction.rs`**: Implements domain interaction dynamics:
  - Radial shockwaves from mouse clicks with smooth inverse-distance impulse and thermal gain.
  - Directional momentum transfer from mouse dragging (fluid stirring).
  - Acoustic harmonic ripples and thermal perturbations from keyboard typing.
  - Convective pressure and buoyancy modulation from mouse scrolling.
- **`field.rs`**: Evaluates the cumulative scalar field at any $(x, y)$ coordinate:
  $$F(x, y) = \sum_{i=1}^{N} \frac{R_i^2}{(x - x_i)^2 + (y - y_i)^2 + \epsilon}$$
- **`simulation.rs`**: Maintains the active blob collection and advances time using explicit $\Delta t$.

### 4.2. Input Translation & Coordinates (`src/input/`)
- **`coords.rs`**: Maps discrete terminal grid cells `(col, row)` to continuous $[0.0, 1.0]$ simulation coordinates with proper vertical inversion.
- **`mouse.rs`**: Tracks mouse dragging vectors and translates raw crossterm `MouseEvent` into domain `Interaction` types.
- **`keyboard.rs`**: Maps key commands and optional keyboard wave ripples.

### 4.3. Virtual Framebuffer & Color (`src/render/`)
- **`framebuffer.rs`**: Holds a 2D grid `Vec<Rgb>` of size `width x height`.
- **`color.rs`**: Provides `Rgb` structs, linear interpolation (`lerp`), and multi-stop gradient color maps.
- **`halfblock.rs`**: Half-block renderer packing two vertical virtual pixels $(x, 2y)$ and $(x, 2y + 1)$ into a single character cell `▀`.
- **`block.rs`**: Full-block renderer mapping single character cells `█`.
- **`braille.rs`**: High-density 2x4 braille dot matrix renderer (`U+2800`..`U+28FF`).

### 4.4. Reactive System Providers (`src/reactive/`)
- **`signals.rs`**: Defines normalized `SystemSignals { cpu_load, memory_usage, battery_level, io_activity }` where each float is in `[0.0, 1.0]`.
- **`linux.rs`**: Native Linux provider reading `/proc/stat` (CPU ticks), `/proc/meminfo` (RAM usage), `/sys/class/power_supply` (battery), and `/proc/diskstats` (I/O).
- **`windows.rs`**: Native Windows provider using Win32 APIs (`GetSystemTimes`, `GlobalMemoryStatusEx`, `GetSystemPowerStatus`, `GetProcessIoCounters`).
- **`macos.rs`**: Native macOS provider using Mach kernel statistics (`host_statistics64`, `HOST_CPU_LOAD_INFO`, `HOST_VM_INFO64`).
- **`provider.rs`**: `SystemProvider` trait and deterministic `MockSystemProvider`.

### 4.5. Terminal Backend & Lifecycle (`src/main.rs`)
- Safely initializes raw mode, alternate screen, and mouse capture via `crossterm`.
- Installs custom panic hooks ensuring terminal state restoration.
- Signal handlers: Unix `SIGINT`/`SIGTERM` via `signal-hook` and Windows console control events (`CTRL_C_EVENT`, `CTRL_CLOSE_EVENT`) via Win32 `SetConsoleCtrlHandler`.

---

## 5. Failure and Error Handling

- All public functions return `Result<T, LavaError>`.
- No uncontrolled panics or `.unwrap()` calls in the runtime loop.
- Graceful degradation: if a platform subsystem (such as audio capture or hardware telemetry) is unavailable or fails, the core simulation continues seamlessly with normalized default baseline signals.
