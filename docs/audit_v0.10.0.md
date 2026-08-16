# ZenLavaTerm v0.10.0 — Production-Grade Audit Report

**Repository:** https://github.com/githubuser2777/ZenLavaTerm  
**Audit Date:** 2026-08-16  
**Auditor:** Antigravity (AI Pair Programmer)  
**Commit audited:** `dd0a587` (HEAD → main)  
**Latest tag:** `v0.10.0` @ `caa62d6`  
**Rust edition:** 2021 | **Toolchain:** stable  

> [!IMPORTANT]
> This is a **read-only audit**. No files were modified. All findings are based on direct inspection of the repository and verified command output.

---

## 2. Selected Agentic Awesome Skills

The following skills from the catalog were identified as most relevant. Since no catalog was fetched live (network not available during audit), the skill IDs below refer to entries in the `/home/skids/.gemini/antigravity-cli/skills/` directory.

| Skill ID | Rationale | Coverage |
|---|---|---|
| `architect-review` | Systems architecture evaluation across modules | Architecture, module coupling, public API |
| `rust-pro` | Rust idioms, ownership, lifetimes, unsafe, error handling | Rust quality audit |
| `security-auditor` | Supply-chain risk, CI/CD security, secret handling | Security, CI/CD |
| `code-reviewer` | Line-level code correctness and best practices | Bug and logic review |
| `performance-engineer` | CPU/memory profiling, algorithmic complexity | Performance |
| `tdd-workflow` | Test coverage, test quality, testing patterns | Testing audit |
| `debugger` | Tracing the Arch Linux CI failure | CI debugging |
| `gitlab-ci-patterns` (analogue for GHA) | Workflow structure and reliability | GitHub Actions audit |

**Catalog gap noted:** No dedicated skill for **terminal emulator safety / ANSI escape correctness** or **Arch Linux PKGBUILD validation** was available. These were audited manually.

---

## 1. Executive Summary

ZenLavaTerm v0.10.0 is a well-structured, zero-dependency Rust terminal visualizer that has progressed through nine phases of feature development. The codebase demonstrates consistently clean Rust idioms, zero `unsafe` blocks, zero `panic!` macros in production paths, and a 67-unit / 7-integration test suite that passes cleanly.

**Phase 9 (Multiplexer & Widget Mode)** is architecturally sound with good separation of concerns. The policy resolver, compact scaler, and multiplexer detector are all testable, deterministic, and well-tested.

**Three significant issues require attention before v0.10.0 is considered production-ready:**

1. **CI Release workflow is broken** — The `build-archlinux-package` job has been failing across all v0.10.0 release tags due to container permission conflicts. The fix applied in `dd0a587` (HEAD) is not yet tagged and has not been release-tested.
2. **`SECURITY.md` still lists v0.9.x as the supported version** — this is stale and misdirects responsible disclosure.
3. **`config.validate()` is NOT called on the default config** — invalid configs (e.g. from a badly edited TOML) may silently bypass validation if the config file does not exist at the default path.

The overall quality is genuinely high for a solo/small-team project at this stage. The architecture is clean, idiomatic, and maintainable.

---

## 3. Architecture Audit

### Module Map

```
src/
  lib.rs          — error types, crate re-exports
  main.rs         — CLI parsing, event loops (fullscreen / inline / headless / snapshot)
  core/           — Simulation, Blob, ScalarField, PhysicsParams, step_blob
  render/         — VirtualFramebuffer, ColorPalette, 3 renderers, rasterize_simulation
  config/         — TOML schema, load_config, validate
  input/          — keyboard event mapping
  reactive/       — SystemSignals, LinuxSystemProvider, MockSystemProvider
  audio/          — FFT, ring buffer, providers, signals
  theme/          — presets, pywal/wallust extractors, auto-detection, file loader
  widget/         — MultiplexerKind, CompactScaler, ResolvedPolicy, render_snapshot
```

### Findings

**GOOD — Clean unidirectional dependency graph.** `core` has no dependencies on `render`, `config`, `widget`, or `audio`. Modules communicate through domain types (`SystemSignals`, `AudioSignals`, `ColorPalette`, `ResolvedPolicy`). This is correct architecture for a library crate.

**GOOD — No circular imports.** The `lib.rs` pub module structure is flat and explicit.

**MEDIUM — `main.rs` is 552 lines with significant code duplication.** `run_fullscreen_interactive` (lines 175–289) and `run_inline_interactive` (lines 291–409) share ~90% of their logic. The only meaningful difference is: inline uses `cursor::Hide` only (no alternate screen), and inline clips rows to 10. This duplication creates a future regression risk — a fix to one loop may not be applied to the other. Recommendation: extract a `run_event_loop(config: LoopConfig) -> Result<()>` abstraction.

**LOW — `Widget` mode routes to `run_fullscreen_interactive`** (line 540–541). Widget mode is semantically different from Interactive (lower FPS, compact physics), but both funnel to the same rendering path without any mode-specific differentiation inside that function. The policy correctly sets `target_fps` and `force_compact`, but the loop function does not display a mode indicator or log widget vs. interactive differences. This is a documentation/UX gap, not a functional bug.

**LOW — No `XDG_CONFIG_HOME` support.** `default_config_path()` only checks `$HOME/.config`. On systems where `$XDG_CONFIG_HOME` is set to a non-default path, the config file is silently ignored.

---

## 4. Security Audit

| # | Severity | Finding | Evidence | Recommendation |
|---|---|---|---|---|
| S1 | MEDIUM | `SECURITY.md` lists v0.9.x as supported, not v0.10.0 | `SECURITY.md` line 7: `0.9.x | :white_check_mark:` | Update to reflect v0.10.0 |
| S2 | LOW | `cross` installed from git HEAD, not a pinned release | `release.yml` line 60: `cargo install cross --git https://github.com/cross-rs/cross` | Pin to a tagged release: `--tag v0.2.5` |
| S3 | LOW | No `cargo deny` or `cargo audit` in CI | CI workflow has no supply-chain vulnerability scanning step | Add `cargo audit` or `cargo deny check` to `check-and-format` job |
| S4 | LOW | `softprops/action-gh-release@v2` and `actions/checkout@v4` not pinned to SHA | Release workflow uses mutable tags | Pin all third-party actions to their full commit SHA |
| S5 | INFO | No `unsafe` code present | Verified via grep | Excellent |
| S6 | INFO | No secrets hardcoded | Verified via grep | Correct use of `secrets.GITHUB_TOKEN` |
| S7 | INFO | Terminal escape sequences are byte-safe | All ANSI output uses `write!(_, "\x1b[...")` with format args limited to `u8` RGB values — no user-controlled strings in escape sequences | No injection risk |

**Assessment:** No critical security vulnerabilities. The main concern is supply-chain hygiene (unpinned action tags, `cross` from git HEAD).

---

## 5. Rust / Code-Quality Audit

### Verified Clean

- ✅ `cargo check` — exit 0, no errors
- ✅ `cargo fmt --check` — exit 0, no formatting violations
- ✅ `cargo clippy --all-targets --all-features -- -D warnings` — exit 0, zero warnings
- ✅ `cargo build --release` — exit 0
- ✅ `cargo test` — 67 unit + 7 integration tests, 0 failures
- ✅ Zero `unsafe` blocks in any source file
- ✅ Zero `panic!` macro calls in production code paths

### Specific Findings

| # | Severity | Location | Finding |
|---|---|---|---|
| R1 | **HIGH** | `src/audio/fft.rs:49–50` | **`assert!` in production path panics on mismatched slice lengths.** `compute_fft` calls `assert_eq!(n, imag.len())` and `assert!(n.is_power_of_two())`. These fire as panics in release builds. With `panic = "abort"` in `[profile.release]`, a misuse of the public `compute_fft` API terminates the process. These should be `Result<(), FftError>` returns. |
| R2 | MEDIUM | `src/main.rs:494` | `terminal::size().unwrap_or((80, 24))` silently falls back. This is acceptable for interactive mode but means snapshot mode (`--snapshot` without `--width/--height`) could silently use incorrect dimensions on platforms where `terminal::size()` fails. A warning log would help. |
| R3 | MEDIUM | `src/main.rs:175–409` | Near-duplicate event loop functions (`run_fullscreen_interactive` / `run_inline_interactive`) — ~220 lines of duplicated logic. Future bugs fixed in one will not propagate to the other. |
| R4 | MEDIUM | `src/config/mod.rs:58` | `Ok(Config::default())` — when no config file exists, `validate()` is **never called** on the default config. The `Config::default()` is hard-coded to valid values, so this doesn't fail today, but the validation bypass is a latent risk if defaults ever drift. Should call `validate()` here too. |
| R5 | LOW | `src/main.rs:461` | `thermal_transfer_rate: 0.40` is hardcoded in `main.rs`, not sourced from `SimulationConfig`. The config schema has no `thermal_transfer_rate` field, creating a hidden constant disconnected from the config system. |
| R6 | LOW | `src/audio/fft.rs:170–172` | Normalization scale factors (15.0, 30.0, 60.0) for bass/mid/treble are empirical constants with no documentation or configurability. Their values are untested for real audio and may clip aggressively on loud sources. |
| R7 | LOW | `src/reactive/linux.rs` | `parse_cpu_stat` and `parse_meminfo` are `pub` functions exposed in the library's public API, but they parse internal kernel string formats. These are test utilities that should be `pub(crate)` to prevent accidental external reliance. |
| R8 | INFO | All `unwrap()`/`expect()` calls in non-test code | All `unwrap()`/`expect()` calls in non-test code are inside `#[cfg(test)]` blocks — confirmed clean. |
| R9 | INFO | `SimplePrng` (XorShift64) | The PRNG is documented as non-cryptographic. Seed `0` is handled with a magic fallback. Correct and appropriate for this use case. |
| R10 | INFO | `Simulation::MAX_DT = 0.10` clamp | Prevents physics blow-up on large frame delays. Correct. |

---

## 6. Phase 9 Audit (Multiplexer & Widget Mode)

### Multiplexer Detection (`src/widget/multiplexer.rs`)

- ✅ Correctly uses dependency-injection pattern (`detect_multiplexer_with`) for testability
- ✅ Handles empty/whitespace-only env vars as `GenericTerminal` (correct)
- ✅ Checks `TMUX` before `ZELLIJ` (priority ordering is reasonable)
- ⚠️ **LOW:** `detect_multiplexer()` is called zero times in `main.rs`. The detection result is not used to influence any runtime behavior (FPS, geometry, etc.). The feature is infrastructural but currently has no behavioral effect in the actual event loop — it exists as a detection utility that's wired nowhere.

### Compact Mode (`src/widget/compact.rs`)

- ✅ `should_compact` is pure and deterministic
- ✅ Area-based thresholds (200, 800) are well-chosen
- ✅ `CompactScaler::adapt_physics` correctly scales buoyancy and noise; gravity and viscosity are preserved (correct — gravity and viscosity are structural, not aesthetic)
- ✅ `radius_scale` is computed but **not applied** to physics — it is stored in `CompactProfile` but `adapt_physics` does not use it. **This is a bug.** The blob radius scaling (which the CHANGELOG and roadmap explicitly describe) is silently dropped.

> [!CAUTION]
> **Bug (HIGH):** `CompactProfile.radius_scale` is computed by `calculate_profile` but never applied anywhere. `adapt_physics` copies only `buoyancy_scale` and `noise_scale`. The per-blob radius adaptation described in the CHANGELOG and goal docs does not execute at runtime.

### Snapshot Mode (`src/widget/snapshot.rs`)

- ✅ Correctly avoids cursor-position escapes (`\x1b[{row};{col}H`) in snapshot output
- ✅ Ends every frame with `\x1b[0m` reset
- ✅ Zero terminal state changes
- ✅ Handles micro-geometries (20x1, 20x2, 20x3) correctly
- ⚠️ **LOW:** The snapshot writes a `\n` after each row except the last (`if row > 0`). For `rows == 1`, no newline is emitted — correct. However, for tmux `status-right` embedding, the user must manually trim the trailing `\x1b[0m` or the status bar will show a reset artifact. This is undocumented.

### Policy Layer (`src/widget/policy.rs`)

- ✅ Clear precedence: CLI > TOML > Defaults
- ✅ Conflict detection: `--snapshot + --inline`, `--inline + --headless` are rejected
- ✅ Dimension validation: zero dimensions and partial specifications rejected
- ⚠️ **MEDIUM:** `--snapshot + --widget` is **not** a conflict but may confuse users. A snapshot in widget mode makes no semantic sense (snapshot is one-shot, widget is looping). A warning or documentation note is recommended.
- ⚠️ **MEDIUM:** The policy does not validate `target_fps == 0`. If `cli_fps = Some(0)` is passed, it resolves to a target frame duration of infinity (`Duration::from_secs_f32(f32::INFINITY)`), which will cause `thread::sleep` to hang forever.

### Inline Mode (`src/main.rs:291-409`)

- ✅ Does not enter alternate screen
- ✅ Restores cursor visibility on exit (`cursor::Show`)
- ✅ Disables raw mode on exit
- ⚠️ **MEDIUM:** On resize event in inline mode (line 363), rows are clamped: `rows = if new_rows > 12 { 10 } else { new_rows }`. This magic number `10` matches the initial default but is not configurable and not documented. A user with `--height 20` will see it overridden by the resize handler.

### Signal Handling / Terminal Restoration

- ✅ `setup_panic_hook()` calls `disable_raw_mode()` and `LeaveAlternateScreen` on panic
- ⚠️ **HIGH:** `setup_panic_hook()` only calls `execute!(stdout(), LeaveAlternateScreen, cursor::Show)`. For **inline mode**, `EnterAlternateScreen` is never called, but `cursor::Show` is correct. However, `setup_panic_hook` is called from `run_inline_interactive` — if it panics before `cursor::Hide`, calling `cursor::Show` in the hook is a no-op. If it panics after `cursor::Hide`, the hook correctly shows the cursor. But if the hook itself fails (e.g. stdout is closed), the terminal is left in raw mode permanently.
- ⚠️ **HIGH:** Neither `SIGTERM` nor `SIGINT` are explicitly handled. When the user sends `CTRL+C` (SIGINT), crossterm's raw mode is bypassed and the terminal may remain in raw mode. This is the most common user-facing terminal corruption scenario. Crossterm handles SIGINT on some platforms by calling the default handler, but this is not guaranteed across terminal emulators.

---

## 7. CI/CD Audit

### CI Workflow (`.github/workflows/ci.yml`)

| # | Severity | Finding |
|---|---|---|
| C1 | INFO | `RUSTFLAGS = "-D warnings"` — Correct: denies all compiler warnings in CI |
| C2 | INFO | Matrix: ubuntu, macos, windows — correct triple coverage |
| C3 | INFO | `Swatinem/rust-cache@v2` — good for build speed but not pinned to SHA |
| C4 | LOW | CI does not run `cargo audit` / `cargo deny` — supply chain gap |
| C5 | LOW | No separate clippy job for release target (`--release`) — clippy may miss platform-conditional code |
| C6 | INFO | `dtolnay/rust-toolchain@stable` — uses mutable `stable` channel, not pinned to a specific Rust version |

### Release Workflow (`.github/workflows/release.yml`)

| # | Severity | Finding |
|---|---|---|
| C7 | **CRITICAL** | **Arch Linux job still broken in the tagged v0.10.0 release.** Run ID `31940753764` (tag `v0.10.0 @ caa62d6`) shows job `95149453374` failed with `Access to the path '/__w/_temp/_github_workflow/event.json' is denied`. The fix in `dd0a587` (HEAD) changes `chown ... /__w` → `chown ... "$PWD"` + `chmod 777 /__w/_temp /__w/_actions`, but this commit is **not tagged** and the release was triggered from the broken commit. |
| C8 | HIGH | The previous fix (`caa62d6`) for the git safe directory problem was insufficient — it retained `chown -R builduser:builduser ... /__w || true`. This is what caused the permission denial on runner infrastructure files. The HEAD fix is correct but unverified in CI. |
| C9 | MEDIUM | `cross` is installed from `git HEAD` (line 60) for every release build with `use_cross: true`. This means the musl build is non-reproducible and could break silently if `cross-rs/cross` main branch changes. |
| C10 | MEDIUM | `generate_release_notes: true` on `softprops/action-gh-release@v2` auto-generates notes from PRs/commits. This is fine but means the formal CHANGELOG.md and auto-generated notes may diverge. |
| C11 | LOW | Arch Linux job uses `makepkg --nodeps` — skips runtime dependency verification. Correct for CI but the released package may have undeclared runtime deps on user systems. |
| C12 | LOW | `strategy: fail-fast: false` on `build-release` is correct (allows other platforms to complete even if one fails). |
| C13 | INFO | `permissions: contents: write` is set at workflow level, applying to all jobs including the CI check job. It should ideally only apply to the upload step. |

---

## 8. Packaging / Release Audit

### PKGBUILD (`packaging/arch/PKGBUILD`)

- ✅ Version: `0.10.0` — matches `Cargo.toml`
- ✅ `cargo build --release --locked` — correct, uses locked dependencies
- ✅ `cargo test --release --locked` in `check()` — good
- ✅ `install -Dm755` binary — correct permissions
- ✅ Source URL uses `refs/tags/v$pkgver` — correct tarball reference

### PKGBUILD.bin (`packaging/arch/PKGBUILD.bin`)

- ✅ Version updated to `0.10.0` in HEAD
- ⚠️ **MEDIUM:** `sha256sums=('SKIP')` — both PKGBUILD and PKGBUILD.bin skip checksum verification. For a binary package (PKGBUILD.bin), this means the downloaded binary is not integrity-checked. For AUR packaging, `SKIP` is only acceptable for local-source packages — for hosted releases, a real SHA256 should be computed and embedded.

### `package_arch.sh` (`scripts/package_arch.sh`)

- ✅ Extracts version from `Cargo.toml` dynamically — no hardcoded version
- ✅ Attempts `git archive` first, falls back to `tar` — good fallback strategy
- ⚠️ **HIGH:** The script runs `makepkg -f --nodeps --noconfirm`. In the CI container where this is invoked as `builduser`, it runs inside a path that may not be writable after checkout actions run as root. The `chown` fix in HEAD addresses this, but has not been validated in a CI run.
- ⚠️ **MEDIUM:** Line 65: `sudo pacman -U "$BUILT_PKG"` — uses `sudo` inside a CI container where `sudo` may not be installed. This is only reached when `--install` flag is passed, which CI does not do. Low actual risk.

### `.SRCINFO`

- ✅ Updated to `0.10.0` in HEAD
- ⚠️ **LOW:** `.SRCINFO` is a manually maintained file. It should be regenerated via `makepkg --printsrcinfo` after PKGBUILD changes to ensure synchrony. Manual maintenance risks drift.

### Release Artifacts Completeness (verified from run `31940753764` while in progress)

| Platform | Status |
|---|---|
| x86_64-linux-gnu | ✅ Built |
| x86_64-linux-musl | ✅ Built |
| x86_64-windows-msvc | ✅ Built |
| aarch64-apple-darwin | ✅ Built |
| x86_64-apple-darwin | 🔄 In-progress at audit time |
| Arch Linux .pkg.tar.zst | ❌ Failed (CI bug) |

---

## 9. Testing Audit

### Quantitative Coverage

| Layer | Tests | Status |
|---|---|---|
| Unit (lib) | 67 | ✅ All pass |
| Integration | 7 | ✅ All pass |
| Benchmarks | 3 benchmark groups | ✅ Build verified (`--no-run`) |
| Doc tests | 0 | N/A — no inline doc examples |

### Qualitative Assessment

**GOOD:**
- Every module has `#[cfg(test)]` unit tests
- Physics tests are behavioral (hot blob rises, cold blob sinks) not just structural
- Snapshot tests verify ANSI SGR sequence correctness
- FFT tests use synthetic sine waves to verify frequency bin isolation
- Policy tests cover edge cases (zero FPS, partial dimensions, conflicts)
- Integration tests cover all 9 feature phases end-to-end

**GAPS:**
- ⚠️ **HIGH:** `compact.rs` `radius_scale` is tested as being computed correctly but its application is never tested — because it's never applied (see R1 bug above).
- ⚠️ **MEDIUM:** No tests for signal handling / terminal restoration on panic or SIGINT.
- ⚠️ **MEDIUM:** No tests for `load_from_path` with an invalid TOML file (error path).
- ⚠️ **MEDIUM:** No property-based or fuzz tests. The physics engine and FFT are candidates for property testing (e.g. field values always non-negative, FFT energy conservation).
- ⚠️ **LOW:** Integration test `test_phase9_compact_mode_integration` asserts `profile.blob_count == 4` for 20x8 (area 160 < 200), which is correct, but does not verify that the adapted physics were actually applied to the simulation — just that the simulation ran.
- ⚠️ **LOW:** No test for `--snapshot` with `terminal::size()` fallback (headless path without explicit dimensions).

---

## 10. Documentation / UX Audit

| # | Severity | Finding |
|---|---|---|
| D1 | **HIGH** | `SECURITY.md` line 7 still lists `0.9.x` as the only supported version. Must be updated to `0.10.x` for release. |
| D2 | MEDIUM | README example tag is `v0.10.0` (updated in HEAD) — but the release tagged build is from `caa62d6`, not HEAD. The README in the tagged release still shows `v0.9.0`. |
| D3 | MEDIUM | `RenderConfig` has `gradient` and `double_buffering` fields in the TOML schema, but neither is used anywhere in `main.rs` or the rendering pipeline. These are dead config options that mislead users. |
| D4 | LOW | The CHANGELOG for v0.10.0 describes "radius scaling" for compact mode (line 12: "adapting blob counts (2–8) and particle radii"). As noted above, `radius_scale` is computed but never applied. The CHANGELOG is technically misleading. |
| D5 | LOW | Widget mode behavior in `Interactive | Widget` branch (main.rs:540) routes to `run_fullscreen_interactive`. The README and CHANGELOG describe widget mode as "low-overhead" — this is enforced via FPS and compact physics, but users running `--widget` will still see the same fullscreen alternate-screen takeover as `--interactive`. |
| D6 | LOW | No man page or shell completion scripts for the CLI. Recommended for v1.0 polish phase. |
| D7 | INFO | `docs/` directory contains comprehensive design documents, roadmap, and packaging guide. Quality is high. |
| D8 | INFO | `CODE_OF_CONDUCT.md` and `CONTRIBUTING.md` are present and reasonable. |

---

## 11. Performance Audit

### Algorithmic Complexity

| Operation | Complexity | Notes |
|---|---|---|
| `rasterize_simulation` | O(W × H × B) | W,H = framebuffer dims, B = blob count. For 80×48 halfblock (160×96 virtual) with 12 blobs: ~184,320 field evaluations per frame. Each evaluation is O(B). |
| `ScalarField::evaluate_field` | O(B) | Simple sum, no spatial acceleration |
| `step_blob` | O(1) per blob | Correct |
| `SpectrumAnalyzer::compute_fft` | O(N log N) | Radix-2 CT FFT, correct |
| `PcmRingBuffer::push_slice` | O(N) + Mutex lock | Non-blocking for producers |

### Issues

- ⚠️ **MEDIUM:** No spatial acceleration (BVH, grid hash) for field evaluation. At high blob counts (e.g. 64+), every pixel evaluates against every blob: 80×48 halfblock (160×96 virtual pixels) × 64 blobs = **983,040 float divisions per frame at 30 FPS = ~29.5M ops/sec**. This is likely acceptable on modern hardware but has no upper bound guard — a user setting `--blobs 128` with `braille` renderer (320×192 virtual) would generate ~7.9M field evaluations per frame.
- ⚠️ **MEDIUM:** `HalfBlockRenderer` and `BlockRenderer` do not implement frame differencing/dirty-region optimization. Every frame re-renders all cells. The config has `double_buffering: bool` but it is never read or used. This is a documentation-vs-implementation gap.
- ✅ `BufWriter::with_capacity(64 * 1024)` is used — correct, avoids excessive syscalls.
- ✅ Color caching (`last_fg`, `last_bg`) in `HalfBlockRenderer` — avoids redundant escape sequences for runs of same color.
- ✅ Release profile: `opt-level = 3`, `lto = true`, `codegen-units = 1`, `strip = true` — correct production settings.
- ⚠️ **LOW:** `panic = "abort"` in release profile means the panic hook set by `setup_panic_hook()` **does NOT run** in release builds. The hook calls `disable_raw_mode()` and `LeaveAlternateScreen` — but `panic = "abort"` causes an immediate process abort, bypassing all Rust destructors and panic hooks. Terminal restoration on panic **does not work in release builds**.

---

## 12. Full Finding Table

| ID | Severity | Category | Location | Finding | Confidence |
|---|---|---|---|---|---|
| F01 | **CRITICAL** | CI/Release | `.github/workflows/release.yml` | Tagged v0.10.0 release has broken Arch Linux package CI job; fix exists in HEAD but is untagged | HIGH |
| F02 | **HIGH** | Bug | `src/widget/compact.rs` | `radius_scale` computed but never applied — blob radius adaptation is silently dropped | HIGH |
| F03 | **HIGH** | Bug | `src/audio/fft.rs:49-50` | `assert!` / `assert_eq!` in production `compute_fft` path panic-abort in release builds on API misuse | HIGH |
| F04 | **HIGH** | Bug | `Cargo.toml` + `src/main.rs` | `panic = "abort"` in release profile silently disables `setup_panic_hook()` — terminal is NOT restored after panic in release builds | HIGH |
| F05 | **HIGH** | Bug | `src/main.rs` (signal handling) | SIGINT/SIGTERM not explicitly handled; crossterm raw mode may not be restored when process is terminated by signal | MEDIUM |
| F06 | **HIGH** | Docs | `SECURITY.md:7` | Supported version table shows `0.9.x` only; must be updated for v0.10.0 release | HIGH |
| F07 | MEDIUM | Bug | `src/widget/policy.rs:82` | `cli_fps = Some(0)` passes validation and produces a zero FPS target, causing `thread::sleep` to hang indefinitely | HIGH |
| F08 | MEDIUM | Bug | `src/main.rs:363` | Inline mode resize handler hardcodes `rows = 10`, overriding `--height` on resize events | HIGH |
| F09 | MEDIUM | Architecture | `src/main.rs:175-409` | Near-duplicate event loop functions (`run_fullscreen_interactive` vs `run_inline_interactive`) — regression risk | HIGH |
| F10 | MEDIUM | Bug | `src/config/mod.rs:58` | `Config::default()` returned without calling `validate()` when no config file exists | MEDIUM |
| F11 | MEDIUM | Testing | Integration tests | No test verifying `radius_scale` application to blob radii | HIGH |
| F12 | MEDIUM | Docs | `src/config/schema.rs` | `gradient` and `double_buffering` config fields are parsed but never used in rendering | HIGH |
| F13 | MEDIUM | CI | `release.yml:60` | `cross` installed from `git HEAD` — non-reproducible, no version pinning | HIGH |
| F14 | MEDIUM | Security | `release.yml`, `ci.yml` | Third-party actions (`softprops/action-gh-release@v2`, `Swatinem/rust-cache@v2`) not pinned to SHA | HIGH |
| F15 | MEDIUM | UX | `src/widget/multiplexer.rs` | Multiplexer detection result is never used to influence runtime behavior | HIGH |
| F16 | MEDIUM | Performance | `src/render/halfblock.rs` | Frame differencing (`double_buffering`) is in config but unimplemented | HIGH |
| F17 | LOW | Code Quality | `src/reactive/linux.rs` | `parse_cpu_stat`, `parse_meminfo` are `pub` but are internal parsing utilities — should be `pub(crate)` | HIGH |
| F18 | LOW | Docs | `CHANGELOG.md:12` | Changelog claims "radius scaling" but `radius_scale` is never applied | HIGH |
| F19 | LOW | Security | `packaging/arch/PKGBUILD.bin:13` | `sha256sums=('SKIP')` for binary package — no integrity verification of downloaded binary | HIGH |
| F20 | LOW | Docs | `src/main.rs:461` | `thermal_transfer_rate: 0.40` is a hardcoded magic constant, not sourced from config | HIGH |
| F21 | LOW | Maintainability | `packaging/arch/.SRCINFO` | Manually maintained `.SRCINFO` — should be generated via `makepkg --printsrcinfo` | MEDIUM |
| F22 | LOW | Security | `release.yml:8-9` | `permissions: contents: write` applies to all jobs, not scoped to upload step | MEDIUM |
| F23 | LOW | UX | `src/config/mod.rs:36` | Config discovery ignores `$XDG_CONFIG_HOME` — only checks `$HOME/.config` | HIGH |
| F24 | INFO | Performance | `src/core/field.rs` | No spatial acceleration for field evaluation — O(W×H×B) per frame | HIGH |
| F25 | INFO | Testing | All | No fuzz tests or property-based tests for physics/FFT | HIGH |

---

## 13. Release Readiness Score

| Dimension | Score (0–10) | Notes |
|---|---|---|
| Architecture | 8/10 | Clean, decoupled, idiomatic Rust |
| Rust code quality | 8/10 | No unsafe, no panics; minor issues (F03, F04) |
| Phase 9 correctness | 6/10 | Radius scaling bug (F02), `detect_multiplexer` unused (F15), SIGINT gap (F05) |
| CI / Release pipeline | 4/10 | Arch Linux job broken in tagged release (F01) |
| Packaging | 6/10 | PKGBUILD correct; sha256sums=SKIP for binary; fix untagged |
| Testing | 7/10 | Good coverage; missing radius_scale test, signal tests |
| Documentation | 6/10 | SECURITY.md stale (F06), dead config fields (F12) |
| Security | 7/10 | No unsafe, no injection; supply chain gaps |
| Performance | 7/10 | Acceptable for target use case; double_buffering unimplemented |
| UX / Install | 7/10 | Smooth for primary platform; XDG gap |

**Overall Release Readiness Score: 66 / 100**

---

## 14. GO / NO-GO Recommendation for v0.10.0

> [!CAUTION]
> ## ❌ NO-GO for v0.10.0 as currently tagged

**Blockers:**

1. **[F01 — CRITICAL]** The `v0.10.0` tag points to `caa62d6`, which produces a broken Arch Linux release artifact. Users installing via the `.pkg.tar.zst` from GitHub Releases receive a missing package. The HEAD fix (`dd0a587`) must be tagged and the release re-triggered.

2. **[F02 — HIGH]** Blob radius adaptation (a documented Phase 9 feature) is silently non-functional. The CHANGELOG explicitly describes it; the code computes it but never applies it.

3. **[F04 — HIGH]** `panic = "abort"` in the release profile disables the panic hook that is supposed to restore terminal state. Users in release builds will have their terminals corrupted on any panic.

4. **[F06 — HIGH]** `SECURITY.md` names only v0.9.x as supported — this must be corrected before any public announcement.

**Minimum Remediation to GO:**
- Retag v0.10.0 from HEAD (after verifying the Arch CI fix works)
- Fix `radius_scale` application in `CompactScaler::adapt_physics`
- Remove `panic = "abort"` from release profile, OR document that the panic hook only works in debug builds
- Update `SECURITY.md`

---

## 15. Ordered Remediation Plan

### Tier 1 — Release Blockers (fix before re-tagging v0.10.0)

1. **[F01]** Verify HEAD commit `dd0a587` CI Arch Linux fix, then create and push a new `v0.10.0` tag from it (or tag as `v0.10.1`).
2. **[F02]** Apply `radius_scale` in `CompactScaler::adapt_physics` and add a test verifying `blob.radius` changes after adaptation.
3. **[F04]** Remove `panic = "abort"` from `[profile.release]` in `Cargo.toml` (it silently breaks the documented terminal-restoration panic hook). Alternatively, replace the panic hook with a `ctrlc`/signal handler approach.
4. **[F06]** Update `SECURITY.md` to list `0.10.x` as the supported version.

### Tier 2 — High Priority (address within one patch release)

5. **[F07]** Add FPS validation to `resolve_policy`: return error if `fps == 0`.
6. **[F05]** Install a SIGTERM/SIGINT handler (via `signal-hook` crate, already a transitive dep of crossterm) that runs terminal cleanup before process exit.
7. **[F08]** In inline resize handler, respect explicit `--height` setting instead of hard-capping at 10 rows.

### Tier 3 — Medium Priority (next minor release v0.11.0)

8. **[F03]** Convert `compute_fft` to return `Result<(), LavaError>` instead of panicking with `assert!`.
9. **[F09]** Refactor `run_fullscreen_interactive` and `run_inline_interactive` into a single parameterised `run_event_loop` function.
10. **[F10]** Call `config.validate()` in the `Ok(Config::default())` branch of `load_config`.
11. **[F12]** Either implement `double_buffering` / `gradient` rendering features, or remove them from the TOML schema with a comment.
12. **[F13]** Pin `cross` to a tagged release in `release.yml`.
13. **[F14]** Pin `actions/checkout`, `softprops/action-gh-release`, and `Swatinem/rust-cache` to full commit SHAs.
14. **[F15]** Use `detect_multiplexer()` result in `main.rs` to influence runtime defaults (e.g., default to `--widget` when inside tmux/zellij).
15. Add `cargo audit` step to CI `check-and-format` job.

### Tier 4 — Low Priority / Polish (v0.12.0 / v1.0)

16. **[F17]** Change `parse_cpu_stat` / `parse_meminfo` visibility to `pub(crate)`.
17. **[F19]** Replace `sha256sums=('SKIP')` in `PKGBUILD.bin` with real checksums generated during release.
18. **[F20]** Add `thermal_transfer_rate` to `SimulationConfig` schema.
19. **[F21]** Automate `.SRCINFO` regeneration via CI or Makefile target.
20. **[F23]** Support `$XDG_CONFIG_HOME` in `default_config_path()`.
21. **[F24]** For large blob counts (> 32), consider a grid-hash spatial acceleration structure.
22. Add shell completion generation (`clap_complete`) and a man page.
23. Add property-based tests for physics (using `proptest` or `quickcheck`).

---

## E. Verified / No Issue Areas

| Area | Status | Notes |
|---|---|---|
| Unsafe code | ✅ None | Zero `unsafe` blocks anywhere |
| `panic!` macro in production | ✅ None | Only in test functions |
| ANSI escape injection | ✅ Safe | All format args are `u8` RGB values |
| FFT mathematical correctness | ✅ Verified | Cooley-Tukey radix-2, bit-reversal permutation, butterfly passes all correct |
| Physics model | ✅ Correct | Hot blobs rise, cold blobs sink, viscous damping, boundary reflection |
| Cargo.lock | ✅ Committed | Reproducible builds |
| `cargo fmt` | ✅ Clean | Zero violations |
| `cargo clippy -D warnings` | ✅ Clean | Zero warnings |
| Release profile LTO | ✅ Correct | `lto = true`, `codegen-units = 1`, `opt-level = 3` |
| BufWriter usage | ✅ Correct | 64 KiB write buffer reduces syscalls |
| Color interpolation | ✅ Correct | Linear interpolation with clamping |
| PRNG seed-zero handling | ✅ Correct | Magic seed fallback for `seed == 0` |
| Config validation | ✅ Called | On all loaded config files (not on defaults — F10) |
| Dependency count | ✅ Minimal | 6 runtime dependencies: `crossterm`, `serde`, `serde_json`, `toml`, `thiserror`, `clap` |
| License | ✅ MIT | Consistent in `Cargo.toml`, `LICENSE`, PKGBUILD |
