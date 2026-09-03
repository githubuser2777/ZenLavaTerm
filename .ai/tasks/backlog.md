# ZenLavaTerm Task Backlog

This file tracks planned work items and provides standardized templates for defining tasks.

---

## 1. Task Lifecycle & Transition Rules

1. **Backlog** (`.ai/tasks/backlog.md`): Defined tasks waiting for prioritization.
2. **Active** (`.ai/tasks/active.md`): Tasks currently being executed by a developer or AI agent (limit to 1-2 concurrent tasks).
3. **Completed** (`.ai/tasks/completed.md`): Verified tasks with passing tests, documentation updates, and git commit references.

---

## 2. Standardized Task Templates

### 2.1 Feature Task Template
```markdown
### [FEAT-XXX] <Feature Title>
- **Status**: Backlog
- **Scope**: `src/<module>/...`, `docs/<section>/...`
- **Goal**: <What capability is being added and why>
- **Architecture Impact**: <How it complies with unidirectional data flow and core isolation>
- **Acceptance Criteria**:
  - [ ] Implementation complete with zero `unwrap()` in production paths
  - [ ] Unit tests added under `src/<module>/tests.rs`
  - [ ] Integration tests added in `tests/integration_test.rs`
  - [ ] Documentation updated in `docs/` and `README.md`
  - [ ] `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` pass
```

### 2.2 Bugfix Task Template
```markdown
### [FIX-XXX] <Bug Title>
- **Status**: Backlog
- **Affected Subsystem**: `core` | `render` | `audio` | `reactive` | `input` | `config` | `widget`
- **Reproduction Steps**:
  1. ...
  2. ...
- **Expected Behavior**: <Expected outcome>
- **Actual Behavior**: <Observed failure or error>
- **Acceptance Criteria**:
  - [ ] Regression test reproducing failure added
  - [ ] Root cause resolved cleanly without breaking architectural boundaries
  - [ ] Full test suite passes (`cargo test`)
  - [ ] Headless validation passes (`cargo run -- --headless --frames 30`)
```

### 2.3 Refactor Task Template
```markdown
### [REFACTOR-XXX] <Refactor Title>
- **Status**: Backlog
- **Target Modules**: `src/...`
- **Motivation**: <Code smells, dead code removal, dependency reduction, or design cleanup>
- **Invariants to Preserve**:
  - [ ] Zero behavior changes in public CLI or configuration
  - [ ] Unidirectional data flow maintained
  - [ ] No regression in passing tests (135+ tests pass)
- **Acceptance Criteria**:
  - [ ] Target code simplified or decoupled
  - [ ] `cargo clippy` and `cargo fmt` clean
  - [ ] All tests pass
```

### 2.4 Performance Task Template
```markdown
### [PERF-XXX] <Optimization Title>
- **Status**: Backlog
- **Hot Path Target**: `core/field.rs` | `render/` | `audio/ring_buffer.rs`
- **Baseline Metric**: <Current execution time / throughput from Criterion>
- **Target Goal**: <Target improvement without breaking correctness or readability>
- **Verification Plan**:
  - [ ] Run `cargo bench --bench field_and_render` before and after changes
  - [ ] Ensure 100% deterministic mathematical results preserved
  - [ ] Document verified speedup in `docs/benchmarks/benchmark_baseline.md`
```

### 2.5 Release Task Template
```markdown
### [REL-XXX] Release v<X.Y.Z>
- **Status**: Backlog
- **Target Version**: `<X.Y.Z>`
- **Pre-Release Checklist**:
  - [ ] Version bumped in `Cargo.toml` (`[package] version = "<X.Y.Z>"`)
  - [ ] `Cargo.lock` updated via `cargo check`
  - [ ] `CHANGELOG.md` updated with release date and categorized notes
  - [ ] Full local validation: `cargo fmt`, `cargo clippy`, `cargo test`, `smoke_test.py`
  - [ ] Tag created: `git tag -a v<X.Y.Z> -m "Release v<X.Y.Z>"`
  - [ ] GitHub release workflow verified in Actions
```

---

## 3. Current Backlog Items

### [FEAT-001] Linux Audio Loopback & Monitor Source Auto-Discovery
- **Status**: Backlog (Target: v1.1.0)
- **Scope**: `src/audio/native.rs`, `src/audio/capture.rs`, `src/config/schema.rs`, `docs/architecture/audio-pipeline.md`, `docs/reference/cli.md`
- **Goal**: Implement automatic discovery and selection of PipeWire, PulseAudio, and ALSA monitor/loopback sources on Linux when `--audio-loopback` is passed, matching the zero-configuration loopback experience on Windows WASAPI.
- **Problem & Context**: 
  On Windows, `--audio-loopback` leverages WASAPI render loopback (`eRender`) automatically. On Linux, `--audio-loopback` currently requires users to manually look up and pass monitor endpoint strings via `--audio-device` (e.g. `lavaterm --audio --audio-device "Default ALSA Output"`).
- **Architecture Impact**: 
  Preserves unidirectional data flow and core isolation. Device discovery resides strictly within `src/audio/native.rs`. If no monitor source is found, gracefully falls back to default audio input or synthetic beats.
- **Acceptance Criteria**:
  - [ ] Implement `find_default_linux_monitor_device()` scanning for monitor endpoints (`monitor`, `PipeWire Media Server`, `PulseAudio`) in `src/audio/native.rs`
  - [ ] Support `--audio-loopback` on Linux by auto-selecting the discovered monitor device
  - [ ] Unit tests for device name matching and fallback logic
  - [ ] Integration tests verifying stream fallback when loopback monitor is unavailable
  - [ ] Update documentation in `docs/architecture/audio-pipeline.md`, `docs/reference/cli.md`, and `docs/troubleshooting/common-issues.md`
  - [ ] Zero compiler warnings (`cargo clippy --all-targets --all-features -- -D warnings`) and clean `cargo fmt`

