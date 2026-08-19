# LavaTerm — Maintenance Policy

## Purpose

Keep LavaTerm healthy after each phase and release without turning maintenance into uncontrolled feature development.

> Detect problems, classify them, create actionable issues, fix approved scope, verify, and release safely.

## Sources of Truth

```text
goal.md             → product direction / architecture
GitHub Issues       → current bugs, work, technical debt
git milestones      → phase / release scope
CHANGELOG.md        → user-facing history
docs/               → technical documentation
source + tests      → actual behavior
```

## Maintenance Layers

### Continuous

Run during normal work:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
git status
```

### Release

Before every release:

```text
acceptance criteria
→ tests
→ lint
→ build
→ relevant benchmarks
→ dependency review
→ docs review
→ CHANGELOG
→ git review
→ tag / release
```

### Periodic

After several releases or when complexity grows, audit:

```text
architecture
dependencies
performance
documentation
test quality
dead code
TODO/FIXME
cross-platform behavior
CI
release hygiene
technical debt
```

Create issues from findings instead of rewriting the repository automatically.

## Bug Triage

Severity:

```text
P0 Critical  → terminal/data safety, catastrophic crash, security-critical issue
P1 High      → core feature broken, severe regression, release blocker
P2 Medium    → normal reproducible bug
P3 Low       → polish / minor compatibility / docs issue
P4 Nice      → optional cleanup or improvement
```

Every meaningful bug should become a GitHub Issue unless fixed directly inside the current issue.

## Issue Quality

Every maintenance issue should contain:

```text
Title
Severity
Context
Observed behavior
Expected behavior
Reproduction
Scope
Acceptance criteria
Tests
Dependencies
```

Avoid vague issues such as `clean code` or `fix stuff`.

## Technical Debt

Track:

```text
TODO / FIXME
temporary abstractions
duplicated logic
platform workarounds
missing tests
benchmark gaps
documentation drift
```

Classify each as:

```text
blocking
important
useful
defer
```

Do not fix every debt item immediately.

## Dependency Maintenance

For dependency changes, consider:

```text
security
maintenance status
API compatibility
binary size
compile time
runtime performance
cross-platform support
license
```

Useful tools when available:

```bash
cargo tree
cargo audit
cargo deny check
```

Do not blindly upgrade every dependency.

## Performance Maintenance

Monitor:

```text
CPU
memory
frame time
terminal output volume
startup latency
allocation behavior
simulation cost
renderer cost
FFT / polling cost
```

Workflow:

```text
measure → identify hotspot → change → measure again
```

Do not optimize based only on intuition.

## Simulation Maintenance

Protect against:

```text
NaN / infinity
exploding velocity
unstable timestep behavior
FPS-dependent physics
unexpected blob behavior
non-deterministic tests
```

Prefer delta-time-based physics and reasonable `dt` clamping.

## Rendering Maintenance

Check:

```text
terminal resize
terminal restore
true-color output
Unicode behavior
small / large terminal sizes
frame diff correctness
flicker
output batching
```

The terminal must always be restored on normal exit and on handled failures where possible.

## Cross-Platform Maintenance

Linux is the primary development target, with first-class native support for Windows and macOS.

Use:

```text
core API
    ↓
platform adapter (Linux / Windows / macOS / Mock)
    ↓
normalized signal (SystemSignals, AudioSignals)
```

Rules:
1. Do not scatter OS-specific logic through simulation code.
2. Keep `core` completely free of OS, hardware, and terminal imports.
3. Native CI matrices must validate compilation and testing on Linux (`ubuntu-latest`), Windows (`windows-latest`), and macOS (`macos-latest`).
4. Signal handling and panic recovery must be verified on both Unix (`signal-hook`) and Windows (`SetConsoleCtrlHandler`).
5. Configuration discovery must support standard XDG, Windows `%APPDATA%` / `%USERPROFILE%`, and macOS `$HOME/Library/Application Support`.
6. Packaging validation must verify all 4 official desktop installers (Linux AppImage, Linux DEB, Windows MSI, macOS Universal DMG) via the 3-tier CI/CD architecture (`ci.yml`, `package.yml`, `release.yml`).

## Documentation Maintenance

When these change, update docs in the same work item:

```text
architecture
CLI
configuration
renderer behavior
dependencies
supported platforms
installation
release behavior
```

At every release verify README, examples, config docs and CHANGELOG against the actual code.

## CI Maintenance

Minimum quality gates:

```text
format
lint
test
build
```

When CI fails:

```text
inspect logs
→ reproduce if possible
→ identify root cause
→ fix
→ rerun
```

## Release Checklist

```text
[ ] all milestone issues complete
[ ] acceptance criteria verified
[ ] cargo fmt --check passes
[ ] cargo clippy passes
[ ] cargo test passes
[ ] cargo build passes
[ ] relevant benchmarks checked
[ ] dependencies reviewed
[ ] docs updated
[ ] CHANGELOG updated
[ ] version/tag correct
[ ] git working tree reviewed
[ ] GitHub Release prepared
```

Do not release from an unknown or unexpectedly dirty state.

## Post-Release

Verify:

```text
release artifact
executable startup
basic renderer
terminal cleanup
newly reported issues
```

## Maintenance Boundaries

Maintenance must not silently become feature development.

Example:

```text
Audit discovers duplicated color mapping
        ↓
Create cleanup issue
        ↓
Schedule it
```

Do not automatically redesign every renderer or theme system around that finding.

## Safe Refactoring

Use:

```text
confirm behavior
→ tests
→ small refactor
→ tests again
```

Avoid giant rewrites of stable components without an explicit issue.

## Maintenance Lifecycle

```text
Detect
  ↓
Classify
  ↓
Create Issue
  ↓
Prioritize
  ↓
Schedule
  ↓
Implement
  ↓
Test
  ↓
Review
  ↓
Close
```

## Cadence

Every coding task:

```text
format + lint + test + build
```

Every phase:

```text
validation + docs check + maintenance audit + release
```

Every few releases:

```text
architecture + dependency + performance + cross-platform + docs + debt audit
```

Before v1.0: perform a full project audit.

## Priority Order

```text
correctness
> security
> terminal/data safety
> stability
> performance
> compatibility
> maintainability
> documentation
> polish
```

## Core Principle

> Maintenance should make LavaTerm healthier, not merely larger.

The goal is to preserve a small, reliable core while enabling controlled growth.
