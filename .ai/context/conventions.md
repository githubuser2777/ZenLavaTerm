# ZenLavaTerm Coding Conventions

This document outlines the coding standards, style rules, error handling conventions, and git workflow expectations for ZenLavaTerm.

---

## 1. Rust Code Style & Idioms

- **Rust Edition**: 2021 edition.
- **Formatting**: Adhere strictly to repository formatting via `rustfmt.toml`. Check with:
  ```bash
  cargo fmt --check
  ```
- **Linting**: Keep code warning-free under `-D warnings`:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  ```
- **Naming Conventions**:
  - Types, Structs, Enums, Traits: `UpperCamelCase` (e.g., `VirtualFramebuffer`, `SpectrumAnalyzer`).
  - Functions, Methods, Variables, Modules: `snake_case` (e.g., `evaluate_at`, `load_config`).
  - Constants & Statics: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_BLOB_COUNT`).

---

## 2. Error Handling & Safety Standards

1. **No Production Panics**:
   - Never use `.unwrap()` or `.expect()` in non-test paths.
   - Do not trigger `panic!()` in runtime loops.
2. **Unified Error Type**:
   - Use `lavaterm::Result<T>` (`Result<T, LavaError>`).
   - Add new variants to `LavaError` in [src/lib.rs](file:///home/skids/Documents/code/ZenLavaTerm/src/lib.rs) if a new error category is required.
   - Implement `std::fmt::Display` and `std::error::Error` manually without introducing extra macro dependencies.
3. **Graceful Fallback**:
   - If optional platform telemetry or audio hardware is missing or errors, fall back to safe synthetic defaults (`SyntheticAudioGenerator`, `MockSystemProvider`) rather than terminating.

---

## 3. Concurrency & Performance Conventions

1. **Hot Path Zero-Allocation**:
   - In per-frame and per-pixel operations (`src/render/`, `src/core/field.rs`), avoid dynamic heap allocations.
   - Reuse pre-allocated buffers (`VirtualFramebuffer::resize_buffer`, `BufWriter`).
2. **SPSC Lock-Free Ring Buffer**:
   - Keep reader operations in `src/audio/ring_buffer.rs` non-blocking.
   - Adhere to the 64-bit Seqlock protocol (`version: AtomicU64`): odd version indicates active write; verify version stability before and after data copy.

---

## 4. Git & Commit Message Conventions

Use **Conventional Commits** format:
- `feat(<scope>): add new capability`
- `fix(<scope>): resolve bug or edge case`
- `refactor(<scope>): code change without feature or bug fix`
- `perf(<scope>): optimize throughput or memory footprint`
- `test(<scope>): add or update unit/integration tests`
- `docs(<scope>): update documentation or diagrams`
- `chore(<scope>): tooling, dependencies, or packaging updates`
- `release(<version>): prepare release artifacts and bump version`

Example scopes: `core`, `render`, `audio`, `reactive`, `input`, `config`, `widget`, `packaging`, `ci`.
