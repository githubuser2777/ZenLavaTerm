# LavaTerm 🌋

[![CI](https://github.com/ZenLavaTerm/lavaterm/actions/workflows/ci.yml/badge.svg)](https://github.com/ZenLavaTerm/lavaterm/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

> **A terminal-native ambient lava lamp & metaball visualizer written in Rust.**

LavaTerm simulates organic, soothing fluid metaballs directly within your terminal window using modern Unicode block rendering (`▀` / `▄`) and True Color (24-bit ANSI) gradients. Designed from the ground up for ambient aesthetic computing, ricing setups, and future system/audio reactive modes.

```text
       .---.          .---.
      /     \        /     \
     |  (o)  |      |   *   |
      \     /        \     /
       '---'  ~~~~~   '---'
           (  L A V A  )
            ~~~~~~~~~~~
```

---

## Features

- 🫧 **Real-Time Metaball Physics**: Buoyancy, gravity, viscosity damping, and subtle thermal noise.
- 🎨 **True-Color Half-Block Rendering**: Converts virtual pixel grids to terminal character cells via `▀` and `▄` sub-cell packing.
- ⚡ **Zero-Overhead Core**: Simulation logic is completely decoupled from terminal I/O for deterministic testing and peak performance.
- 🛠️ **Configurable**: Fully customizable via TOML files and intuitive CLI flags.
- 🛡️ **Robust Terminal Handling**: Safe raw mode transitions, alternate screen support, and automatic terminal state restoration on exit or panic.

---

## Quick Start

### Prerequisites

- [Rust 1.75+](https://www.rust-lang.org/tools/install) (`cargo`, `rustc`)
- A terminal emulator supporting 24-bit True Color (Alacritty, Kitty, WezTerm, Ghostty, iTerm2, Windows Terminal, GNOME Terminal, etc.)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/ZenLavaTerm/lavaterm.git
cd lavaterm

# Build in release mode
cargo build --release

# Run LavaTerm
cargo run --release

# Run headless simulation test (no TTY takeover)
cargo run -- --headless --frames 30
```

---

## Usage & CLI Options

```text
Usage: lavaterm [OPTIONS]

Options:
  -c, --config <PATH>        Path to custom TOML configuration file
  -r, --renderer <TYPE>      Renderer backend: halfblock | block [default: halfblock]
      --fps <FPS>            Target frames per second [default: 30]
      --blobs <COUNT>        Number of metaball blobs [default: 12]
      --headless             Run headless simulation without taking over TTY (useful for testing/CI)
      --frames <COUNT>       Number of frames to step when in headless mode [default: 60]
  -h, --help                 Print help
  -V, --version              Print version
```

---

## Configuration

LavaTerm can be configured using a simple TOML configuration file.

```toml
[simulation]
blobs = 12
gravity = 0.12
buoyancy = 0.80
viscosity = 0.93
noise = 0.15

[render]
renderer = "halfblock"
fps = 30
gradient = true

[palette]
bottom = "#ff3b00"
middle = "#ff7a00"
top = "#7b2cff"
```

For more details on configuration parameters, see [docs/configuration.md](docs/configuration.md).

---

## Architecture

LavaTerm follows a strict unidirectional data flow:

```text
Signals / Time / Input
         ↓
  Simulation Core (Blobs & Scalar Field)
         ↓
  Virtual Framebuffer (2D RGB Canvas)
         ↓
  Terminal Renderer (Half-block / Block)
         ↓
  Terminal Output (ANSI batched stream)
```

See [docs/architecture.md](docs/architecture.md) for detailed architectural specifications.

---

## Documentation

- [Architecture & Data Flow](docs/architecture.md)
- [Simulation & Physics Model](docs/simulation.md)
- [Rendering Pipeline & Terminal I/O](docs/rendering.md)
- [Configuration Schema](docs/configuration.md)
- [Development Roadmap](docs/roadmap.md)
- [Contributing Guidelines](docs/contributing.md)

---

## Contributing

Contributions are welcome! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before submitting pull requests or issues.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
