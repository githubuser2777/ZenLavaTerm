# Terminal UI Architecture & Multiplexer Widget Integration

ZenLavaTerm (`lavaterm`) features a **terminal-native TUI architecture**. This document details the UI design, execution modes, multiplexer integration, and explicitly contrasts this architecture with GUI frameworks such as Tauri.

---

## 1. Terminal-Native TUI vs GUI / Tauri

### 1.1 Architectural Clarification
While many modern visualizers use webview wrappers such as Tauri or Electron to render HTML5 canvases via GPU acceleration, **ZenLavaTerm is strictly a terminal-native ANSI TUI application**:

| Architectural Dimension | ZenLavaTerm TUI Architecture | GUI / Tauri Alternative |
|---|---|---|
| **Runtime Environment** | Pure terminal (TTY), SSH sessions, virtual consoles | Desktop windowing server (X11, Wayland, Quartz, Win32 DWM) |
| **Rendering Primitive** | ANSI True-Color escape codes (`▀`, `█`, Braille `U+2800`) | WebKit / WebView2 / Blink rendering HTML5 canvas |
| **Binary Footprint** | ~3.8 MB self-contained binary | Often 15 MB to 80+ MB bundle with webview bridge |
| **Startup Latency** | < 10 milliseconds | 200 ms to 1,500 ms (webview bootstrap) |
| **CPU / Battery Footprint** | < 1.5% CPU at 60 FPS | Higher base overhead due to browser runtime |
| **Headless / Automation** | First-class (`--headless`, `--snapshot`) | Requires virtual display servers (e.g. Xvfb) |

ZenLavaTerm's design prioritizes lightweight terminal integration, making it suitable for ambient backgrounds, tiling window managers, and multiplexers where heavy GUI runtimes would be unacceptable.

---

## 2. Terminal Lifecycle & Event Management (`src/main.rs`)

1. **Terminal Setup**:
   - Switches standard output to raw mode using `crossterm::terminal::enable_raw_mode()`.
   - Enters alternate screen buffer via `EnterAlternateScreen` (unless `--inline` or `--snapshot` is specified).
   - Hides cursor and enables mouse event tracking (`EnableMouseCapture`).
2. **Crash & Signal Resilience**:
   - Installs a custom Rust panic hook that restores standard terminal mode, disables mouse tracking, and shows the cursor before printing backtraces.
   - Listens to Unix POSIX signals (`SIGINT`, `SIGTERM`) and Windows console events (`CTRL_C_EVENT`, `CTRL_CLOSE_EVENT`) to ensure the terminal is never left in an un-usable corrupted state.

---

## 3. Widget Engine & Execution Modes (`src/widget/`)

ZenLavaTerm provides an adaptive policy engine (`src/widget/policy.rs`) that dynamically adjusts rendering parameters based on terminal dimensions or explicit execution modes:

```rust
pub enum ExecutionMode {
    Normal,    // Fullscreen alternate screen, standard 60 FPS, interactive
    Compact,   // Scaled physics profile for small panes or sidebars
    Widget,    // Low-overhead ambient mode (defaults to 15 FPS, compact physics)
    Snapshot,  // Single-frame ANSI rendering to stdout, exits immediately
}
```

### 3.1 Compact Geometry Scaling (`src/widget/compact.rs`)
When terminal dimensions drop below threshold thresholds ($< 60$ columns or $< 18$ rows), `should_compact()` activates `CompactScaler`:
- Reduces the effective number of blobs to avoid overcrowding.
- Scales blob radii dynamically to maintain proportional spacing.
- Adjusts physics viscosity and convection parameters to keep animation fluid in confined spaces.

### 3.2 Snapshot Mode (`src/widget/snapshot.rs`)
- Invoked via `lavaterm --snapshot`.
- Instantiates the simulation, evaluates a single frame, formats ANSI characters, flushes to `stdout`, and terminates without entering alternate screen or raw mode.
- Enables embedding live lava lamp snapshots in:
  - Tmux / Zellij status bars.
  - Waybar, Polybar, or i3blocks widgets.
  - Terminal greeting screens (e.g., `.bashrc`, `.zshrc`, fastfetch motd).
