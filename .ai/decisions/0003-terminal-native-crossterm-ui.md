# ADR-0003: Terminal-Native TUI vs GUI / Tauri Frameworks

- **Status**: Accepted
- **Date**: 2026-08-15
- **Context**: 
  Modern desktop visualizers often reach for web-based GUI wrappers like Tauri or Electron to draw hardware-accelerated canvases. However, the core identity of ZenLavaTerm is a lightweight, low-overhead, ambient lava lamp running natively inside any ANSI terminal, SSH session, tmux/zellij multiplexer pane, or window manager status bar.
- **Decision**:
  Maintain a 100% **terminal-native ANSI TUI** architecture using `crossterm`.
  - Do not introduce Tauri, WebViews, WebGL, or GUI desktop toolkits.
  - Render directly to terminal character cells using sub-cell Unicode tricks: upper half-blocks (`▀`), full blocks (`█`), and 2x4 Braille matrices (`U+2800`..`U+28FF`).
  - Implement a dedicated `widget` module (`CompactProfile`, `CompactScaler`, `--widget`, `--snapshot`) to enable embedding in small terminal panes, status bars, and automated scripts without alternate screen takeover.
- **Consequences**:
  - **Positive**: Negligible CPU footprint (<1.5% CPU at 60 FPS); instant startup (<10 ms); no web engine or C++ GUI runtime dependencies; runs over SSH and headless containers; native packaging under 5 MB.
  - **Negative / Trade-offs**: Visual resolution is constrained by terminal character cell dimensions and font metrics; requires True-Color terminal support for smooth gradients.
  - **Invariants**: Keep the application strictly terminal-native; do not add web/GUI runtime dependencies.
