# Testing Strategy & Automated Test Architecture

ZenLavaTerm employs a multi-tiered testing strategy designed to verify mathematical fluid dynamics, multi-threaded audio concurrency, cross-platform system telemetry, and terminal lifecycle management without requiring an interactive human terminal session.

---

## 1. Test Hierarchy Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                 Continuous Integration (CI)                 │
│  - Linux x86_64, macOS Apple Silicon/Intel, Windows x86_64 │
│  - cargo fmt, clippy -D warnings, cargo audit               │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│             PTY & Headless Smoke Test Suites                │
│  - scripts/smoke_test.py (PTY allocation, signals, CLI args)│
│  - cargo run -- --headless --frames 30                      │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│             Integration Tests (tests/*.rs)                  │
│  - tests/integration_test.rs (23 integration tests)         │
│  - tests/common/mod.rs (MockAudioStreamFeeder test fixture) │
│  - Multi-threaded SPSC Seqlock contention tests             │
│  - End-to-end pipeline (PCM samples -> FFT -> Framebuffer)  │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│             Unit Tests (src/**/tests.rs)                    │
│  - 112 focused unit tests across all internal modules       │
│  - Scalar field math, buoyancy, coordinate transformations  │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Unit Testing Layer (`src/`)

Unit tests reside in nested `tests` submodules alongside the code they verify. They focus on:
- **Core Math & Physics**: Validates scalar field potential falloff with distance, superposition, thermal buoyancy rising/sinking, and energy bounds in `src/core/`.
- **Coordinate Transformations**: Validates discrete cell coordinate to continuous $[0.0, 1.0]$ mapping, boundary clamping, and zero-dimension safety in `src/input/coords.rs`.
- **Telemetry Parsing**: Validates parsing of Linux `/proc/stat`, `/proc/meminfo`, `/proc/diskstats`, and battery paths under both normal and missing file conditions in `src/reactive/`.
- **Configuration Validation**: Validates TOML parsing, legacy field aliases, and palette deserialization in `src/config/`.

---

## 3. Integration Testing Layer (`tests/`)

End-to-end integration tests in `tests/integration_test.rs` verify cross-subsystem interactions:
- **SPSC Seqlock Snapshot Coherence**: Spawns concurrent producer and consumer threads to hammer the circular ring buffer across wrap-around boundaries, asserting zero tear detection failures.
- **Hardware Disconnect & Resumption**: Simulates driver disconnections via atomic stream controls, verifying seamless fallback to `SyntheticAudioGenerator` and live resumption.
- **Test Fixture Isolation**: Test harnesses like `MockAudioStreamFeeder` (~230 LOC) are isolated in `tests/common/mod.rs`, preventing test code from bloating the production binary.

---

## 4. Headless & PTY Smoke Testing

- **Headless Runtime Validation**:
  ```bash
  cargo run -- --headless --frames 30
  ```
  Runs the full event loop for 30 ticks without alternate screen switching or terminal raw mode. Essential for headless CI runners.
- **PTY Smoke Suite (`scripts/smoke_test.py`)**:
  Allocates real pseudo-terminals (PTYs) via Python to verify terminal resizing, POSIX signals (`SIGINT`), CLI arguments, and single-frame snapshot output.
