# ZenLavaTerm Current State

> **Last Updated**: 2026-09-03
> **Current Version**: `1.0.1` (see `Cargo.toml`, git tag `v1.0.1`)

---

## 1. Release & Codebase Status

- **Crate Version**: `1.0.1`
- **Git HEAD**: `v1.0.1` (`898db12`)
- **Active Test Suite**:
  - Unit tests: **112 passed**, 0 failed
  - Integration tests: **23 passed**, 0 failed
  - Total automated tests: **135 passed** (`cargo test`)
- **Code Quality Checks**:
  - `cargo fmt --check`: Clean (no formatting differences)
  - `cargo clippy --all-targets --all-features -- -D warnings`: Clean (zero warnings)
- **Crate Size & Dependencies**:
  - Minimal direct dependencies (7 runtime, 1 dev): `crossterm`, `serde`, `serde_json`, `toml`, `clap`, `signal-hook`, `cpal`, `criterion`.
  - Removed direct `thiserror` dependency in v1.0.1 (standard `std::error::Error` implementation).
  - Test harness `MockAudioStreamFeeder` isolated to `tests/common/mod.rs`.

---

## 2. Platform Support & Verification

| Platform | Target Architecture | Audio Backend | Telemetry Provider | Verification Method |
|---|---|---|---|---|
| **Linux** | `x86_64-unknown-linux-gnu` | ALSA (`cpal`) | `/proc/stat`, `/proc/meminfo`, `/sys/class/power_supply` | GitHub Actions Ubuntu runner (CI, smoke tests, AppImage/DEB packaging) |
| **Windows** | `x86_64-pc-windows-msvc` | WASAPI (`cpal` + loopback) | Win32 APIs (`GetSystemTimes`, `GlobalMemoryStatusEx`) | GitHub Actions Windows runner (CI, MSI WiX packaging) |
| **macOS** | `aarch64-apple-darwin` / `x86_64-apple-darwin` | CoreAudio (`cpal`) | Mach kernel (`host_statistics64`) | GitHub Actions macOS runner (CI, Universal DMG packaging) |

---

## 3. Empirical Performance Benchmarks

Empirical Criterion measurements recorded in [docs/benchmarks/benchmark_baseline.md](file:///home/skids/Documents/code/ZenLavaTerm/docs/benchmarks/benchmark_baseline.md):

- **Scalar Field Potential Evaluation**:
  - 6 blobs ($80 \times 20$ grid): **422.79 ns** (~3.78M evaluations/sec)
  - 12 blobs ($80 \times 20$ grid): **423.34 ns** (~3.77M evaluations/sec)
  - 24 blobs ($80 \times 20$ grid): **425.85 ns** (~3.75M evaluations/sec)
- **Framebuffer Rasterization**:
  - Smooth gradient ($80 \times 48$): **127.14 µs** (~**7,865 FPS** throughput)
  - Stepped gradient ($80 \times 48$): **62.68 µs** (~**15,954 FPS** throughput)
  - High resolution ($120 \times 60$): **236.65 µs** (~**4,225 FPS** throughput)
- **Terminal Renderers ($80 \times 48$)**:
  - Half-Block: **83.85 µs** (~11,926 FPS)
  - Block: **81.24 µs** (~12,309 FPS)
  - Braille ($160 \times 96$ subpixels): **72.33 µs** (~13,825 FPS)

---

## 4. Known Verification Caveat

- **Windows WASAPI Headless Audio**:
  - GitHub Actions `windows-latest` virtual machines lack physical audio hardware and render endpoints. CI verifies device enumeration, configuration, and in-memory mock frame pipelines, but live loopback capture requires physical Windows hardware with active audio output.
