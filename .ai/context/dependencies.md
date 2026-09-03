# ZenLavaTerm Dependencies Context

ZenLavaTerm maintains a strict dependency minimization policy to ensure fast compile times, small binary footprint, and auditability.

---

## 1. Direct Rust Dependencies (`Cargo.toml`)

| Crate | Version | Category | Purpose & Usage |
|---|:---:|---|---|
| **`crossterm`** | `0.29` | Runtime | Terminal raw mode, alternate screen switching, mouse event capture, and cursor controls. |
| **`serde`** | `1.0` (with `derive`) | Runtime | Data serialization and deserialization for configuration and theme files. |
| **`serde_json`** | `1.0` | Runtime | Parsing Pywal, Wallust, and custom JSON color palette files. |
| **`toml`** | `0.8` | Runtime | Parsing user configuration files (`config.toml`). |
| **`clap`** | `4.5` (with `derive`) | Runtime | Command-line argument parsing and help text generation. |
| **`signal-hook`** | `0.4` | Runtime | Unix POSIX signal handling (`SIGINT`, `SIGTERM`, `SIGWINCH`). |
| **`cpal`** | `0.18` | Runtime | Cross-platform audio capture streams (ALSA on Linux, WASAPI on Windows, CoreAudio on macOS). |
| **`criterion`** | `0.8` (`default-features = false`) | Dev Only | Micro-benchmarking for scalar field math, rasterization loops, and audio ring buffer throughput. |

---

## 2. Recent Dependency Pruning (v1.0.1)

In release `v1.0.1`, the dependency tree was actively pruned:
- **`thiserror` Removed**: Eliminated direct macro dependency by implementing `std::fmt::Display` and `std::error::Error` manually on `LavaError`.
- **AST Config Rewriter Removed**: Replaced manual AST rewriting with native Serde field aliases (`#[serde(alias = "...")]`), eliminating unused code.
- **Test Harness Isolated**: Isolated `MockAudioStreamFeeder` (~230 LOC) out of the production binary into integration test fixtures (`tests/common/mod.rs`).

---

## 3. System & External Build Dependencies

### Linux:
- `libasound2-dev` / `alsa-lib-devel`: Required by `cpal` for ALSA audio endpoint bindings.
- `pkg-config`: Required by Cargo to locate system ALSA C headers and libraries.

### Windows:
- MSVC Build Tools (`x86_64-pc-windows-msvc`).
- `WiX Toolset` (v3.11 or v3.14): Required to compile `.wxs` source into Windows `.msi` installers (`candle.exe`, `light.exe`).

### macOS:
- Apple Silicon (`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`) SDKs.
- `lipo`: Required to merge dual-architecture binaries into a single Universal Binary.
- `hdiutil`: Required to package the `.app` bundle into a distributable `.dmg` disk image.
