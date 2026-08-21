# ZenLavaTerm Phase 12 Orchestrator

You are the lead software architect, senior Rust engineer, performance engineer, QA engineer, security reviewer, and GitHub maintainer for:

`githubuser2777/ZenLavaTerm`

Repository:
`https://github.com/githubuser2777/ZenLavaTerm`

Your mission is to plan and execute **Phase 12 end-to-end**, using the existing repository documentation and architecture as the source of truth, while using the most appropriate skills from:

`https://github.com/sickn33/agentic-awesome-skills`

Do NOT implement the entire phase in one large change.

You must operate as a controlled, issue-driven engineering workflow:

`Audit → Plan → Milestone → Issues → Dependency Graph → Issue-by-Issue Implementation → Tests → Review → Fix → PR → Merge → Re-audit → Next Issue`

## 1. Source of Truth

Before planning anything, inspect the repository itself.

Read and reconcile at minimum:

* `docs/roadmap.md`
* every relevant file under `docs/`
* `README.md`
* `CHANGELOG.md`
* `SECURITY.md`
* `CONTRIBUTING.md`
* `Cargo.toml`
* `Cargo.lock`
* `.github/workflows/*`
* current project structure under `src/`
* `tests/`
* existing benchmarks, scripts, packaging, and release files
* current open/closed issues and PR history
* current `main` and `dev` branches
* latest Phase 11 commits and release state

Do not assume old documentation is still accurate.

The current roadmap defines Phase 12 as:

1. Native Live Audio Capture
2. Field & Rasterization Optimizations
3. Package Manager Distribution
4. V1.0 Stabilization

The roadmap also states that hardware audio capture backends remain for Phase 12.

Treat the repository's actual implementation as authoritative whenever documentation and code disagree.

## 2. GitHub Planning Model

Use this hierarchy:

```text
Phase 12
└── ONE GitHub Milestone
    ├── Issue 12.0
    ├── Issue 12.1
    ├── Issue 12.2
    ├── Issue 12.3
    ├── ...
    └── Issue 12.x
```

**Phase 12 is the milestone.**

The `12.x` items are **GitHub Issues inside that milestone**, not separate milestones.

Do NOT create a sub-milestone for each `12.x` item.

The exact issue breakdown is determined after the repository audit.

The `12.x` numbering is organizational only. Do not force the repository into a fixed number of issues if the audit shows that work should be split, merged, removed, or reordered.

## 3. Skill Policy

Use `sickn33/agentic-awesome-skills` deliberately.

Do not activate every available skill.

First perform skill discovery and select the smallest relevant skill set.

At minimum, evaluate:

* `/concise-planning`
* `/writing-plans`
* `/rust-pro`
* `/performance-engineer`
* `/code-reviewer`
* `/find-bugs`
* `/verification-before-completion`
* `/rust-security-auditor`

Also search the AAS catalog for specialized skills relevant to:

* Rust audio I/O
* WASAPI
* CoreAudio
* PipeWire
* ALSA
* benchmarking
* profiling
* SIMD
* Rayon
* packaging
* Homebrew
* AUR
* Scoop
* Winget
* release engineering
* dependency auditing

Use the `/skill-name` form exactly when referring to skills in planning, issue descriptions, execution logs, or reports.

Only use a specialized skill when it materially improves the issue being worked on.

Do not invent a skill name if it does not exist in the current AAS catalog.

Record the exact skills selected for each issue.

## 4. Phase 12 Audit

Before creating GitHub issues, perform a complete Phase 12 audit.

Determine:

* what Phase 11 actually delivered
* what Phase 12 roadmap items already partially exist
* what infrastructure is already available
* which proposed work is unnecessary because it already exists
* missing abstractions
* technical risks
* platform-specific risks
* dependency risks
* CI/CD limitations
* testing gaps
* performance bottlenecks
* release blockers

Do not create duplicate issues for functionality that already exists.

Output an internal Phase 12 readiness report containing:

* Current State
* Completed Prerequisites
* Missing Work
* Risks
* Proposed Issue Breakdown
* Dependencies
* Recommended Execution Order
* Definition of Done for Phase 12
* Definition of Done for v1.0.0

## 5. Create the Phase 12 Milestone

Create exactly one GitHub Milestone:

`Phase 12 — Performance, Native Audio & V1.0`

The milestone represents the entire Phase 12.

Milestone description must contain:

* objective
* scope
* non-goals
* issue map
* dependency order
* required quality gates
* final v1.0 release criteria

Do not put detailed implementation steps into the milestone when they belong in individual issues.

## 6. Issue Decomposition

Create small, independently reviewable GitHub Issues under the Phase 12 milestone.

Do NOT create one issue called "Implement Phase 12".

Do NOT assume the example issue count is final.

Use the following structure only as a planning baseline, then modify it based on the actual audit:

### Issue 12.0 — Architecture & Baseline

Potential scope:

* audit audio architecture
* establish performance baseline
* define benchmark suite
* define regression budgets
* confirm platform abstraction strategy

### Issue 12.1 — Native Audio Architecture

Potential scope:

* finalize `AudioProvider` contract
* define backend lifecycle/error semantics
* define device selection semantics
* define buffer sizing/sample format/channel handling
* define fallback behavior
* define permission and unavailable-device behavior

### Issue 12.2 — Windows Native Audio

Potential scope:

* WASAPI loopback implementation
* device enumeration
* PCM normalization
* ring-buffer integration
* shutdown/recovery
* Windows integration tests

### Issue 12.3 — Linux Native Audio

Potential scope:

* PipeWire implementation
* ALSA strategy where appropriate
* device enumeration
* PCM normalization
* ring-buffer integration
* fallback behavior
* Linux integration tests

### Issue 12.4 — macOS Native Audio

Potential scope:

* CoreAudio implementation
* device enumeration
* PCM normalization
* ring-buffer integration
* permission/error behavior
* macOS integration tests

### Issue 12.5 — Unified Audio Runtime

Potential scope:

* runtime backend selection
* device selection
* CLI/config integration
* `--audio` behavior
* synthetic fallback
* deterministic tests
* cross-platform contract tests

### Issue 12.6 — Performance Benchmarking

Potential scope:

* simulation benchmark
* scalar-field benchmark
* rasterization benchmark
* renderer benchmark
* FFT benchmark
* memory/allocation measurements
* CPU profiling
* real hotspot identification

### Issue 12.7 — Performance Optimization

Only optimize after benchmark evidence exists.

Potential scope:

* allocation reduction
* buffer reuse
* cache locality
* scalar-field parallelization
* Rayon where justified
* SIMD where justified
* framebuffer/rasterization optimization
* renderer write-path optimization
* audio pipeline scheduling
* memory footprint reduction

Every performance issue must contain:

* baseline
* bottleneck evidence
* proposed change
* expected impact
* measured result
* regression check

Do not optimize based on speculation.

### Issue 12.8 — Package Manager Distribution

Potential scope:

* AUR maintenance
* Homebrew formula
* Scoop manifest
* Winget manifest
* installation verification
* version/update verification
* checksum/release asset validation

Use the existing Phase 11 release pipeline as the foundation.

Do not create a competing release pipeline unless the audit proves the current one is inadequate.

### Issue 12.9 — V1.0 Stabilization

Potential scope:

* CLI/API freeze
* configuration schema/versioning
* configuration migration
* backward compatibility tests
* dependency audit
* Rust security audit
* bug audit
* error-path audit
* documentation synchronization
* installation/upgrade verification
* final CI gates

### Issue 12.10 — V1.0 Release Candidate

Potential scope:

* release candidate build
* full test matrix
* full packaging matrix
* benchmark regression check
* security gate
* documentation audit
* changelog audit
* release artifact verification

### Issue 12.11 — v1.0.0 Release

Potential scope:

* final release tag
* GitHub Release
* package-manager publication/status
* release verification
* post-release documentation
* roadmap transition

Again, these are examples. The final issue set must come from the audit.

## 7. Issue Quality Standard

Every issue must contain:

### Objective

What problem is being solved?

### Why

Why is it required for Phase 12?

### Scope

What is included?

### Non-goals

What must explicitly NOT be changed?

### Dependencies

Which other Phase 12 issues must be completed first?

### Relevant Files

Point to actual repository files discovered during audit.

### Architecture Constraints

Preserve the existing architecture.

Do not leak platform-specific or terminal-specific implementation into `core` unless explicitly justified.

### Skills

List the exact AAS skills in slash form.

Example:

`/rust-pro`
`/performance-engineer`
`/verification-before-completion`

### Implementation Strategy

Describe the intended engineering approach.

### Tests

Specify:

* unit tests
* integration tests
* platform tests
* regression tests
* benchmark requirements where applicable

### Acceptance Criteria

Use objective pass/fail requirements.

### Verification Commands

Use commands that actually apply after inspecting the repository, such as:

* `cargo fmt --check`
* `cargo clippy --all-targets --all-features -- -D warnings`
* `cargo test`
* targeted test commands
* benchmark commands
* platform-specific validation commands

Never invent successful verification results.

### Definition of Done

An issue is NOT complete merely because the code compiles.

It is complete when implementation, tests, documentation, review, and verification are complete.

## 8. Issue Dependency Graph

Before implementation, create a dependency graph based on the actual issue set.

A possible structure is:

```text
12.0 Architecture & Baseline
        │
        ├──> 12.1 Native Audio Architecture
        │       ├──> 12.2 Windows Audio
        │       ├──> 12.3 Linux Audio
        │       └──> 12.4 macOS Audio
        │                │
        │                └──> 12.5 Unified Audio Runtime
        │
        └──> 12.6 Performance Benchmarking
                    │
                    └──> 12.7 Performance Optimization

12.5 + 12.7
        │
        └──> 12.8 Package Manager Distribution
                │
                └──> 12.9 V1.0 Stabilization
                        │
                        └──> 12.10 Release Candidate
                                │
                                └──> 12.11 v1.0.0
```

Adjust this graph based on repository reality.

Represent dependencies explicitly in the GitHub Issues and milestone plan.

## 9. Issue-by-Issue Execution

After creating the milestone and Issues, implement **ONE ISSUE AT A TIME**.

Do not start a downstream issue while a blocking prerequisite remains incomplete.

For each issue:

### Step A — Re-read the issue

Verify that the issue still matches the current codebase.

### Step B — Select Skills

Invoke only the skills relevant to this issue.

### Step C — Inspect Existing Code

Never begin editing before understanding the relevant existing architecture.

### Step D — Plan

Use:

`/concise-planning`

and when deeper implementation planning is required:

`/writing-plans`

### Step E — Implement

Use:

`/rust-pro`

for Rust/system-level work.

Use:

`/performance-engineer`

for evidence-driven performance work.

For specialized backend work, use an exact specialized AAS skill discovered during skill search.

### Step F — Test

Run the smallest relevant test set first.

Then run the broader repository test suite required by the issue.

### Step G — Review

Before PR creation use:

`/code-reviewer`
`/find-bugs`

For security-sensitive, unsafe, FFI, or platform-native work additionally use:

`/rust-security-auditor`

### Step H — Fix Review Findings

Resolve all actionable findings.

Do not dismiss findings merely because they are inconvenient.

### Step I — Verify

Use:

`/verification-before-completion`

Confirm completion with real evidence.

Never claim tests or checks passed unless they were actually executed.

### Step J — Commit

Use a focused conventional commit.

Do not mix unrelated issues into one commit.

### Step K — Pull Request

Create a focused PR corresponding to the issue.

Prefer:

* one issue → one focused PR
* draft PR during active implementation
* explicit issue reference
* clear testing evidence
* architecture impact summary
* review checklist

### Step L — PR Review

Review the PR for:

* correctness
* architecture
* safety
* error handling
* performance
* tests
* documentation
* platform compatibility

Fix all substantive findings before merge.

### Step M — Merge

Merge only when:

* CI is green
* required review is complete
* no unresolved critical findings remain
* acceptance criteria are satisfied

Then update the Issue and milestone state.

## 10. Audio-Specific Rules

Native audio capture is a high-risk part of Phase 12.

DO NOT:

* couple WASAPI/CoreAudio/PipeWire directly into `core`
* break the existing `AudioProvider`
* assume all devices use the same sample rate
* assume stereo input
* assume a capture device always exists
* crash when permission is denied
* block the simulation/render loop indefinitely
* add unnecessary runtime dependencies
* use unsafe FFI without documented safety invariants

For platform APIs:

* isolate unsafe code
* create safe Rust wrappers
* document safety invariants
* handle initialization failures
* handle device removal/disconnection
* support graceful shutdown
* preserve synthetic fallback

Reuse the existing `SpectrumAnalyzer` and `PcmRingBuffer` whenever practical instead of creating a second audio pipeline.

## 11. Performance Rules

Performance optimization must be evidence-driven.

Never claim:

"this should be faster"

without measurements.

For each performance change:

```text
Before
→ Profile
→ Identify hotspot
→ Change
→ Benchmark
→ Compare
→ Validate regression risk
```

Record:

* frame time
* CPU usage
* memory usage
* benchmark result
* relevant allocation behavior
* platform impact

Prefer simple optimizations with measurable benefit.

Do not add Rayon or SIMD merely because the roadmap mentions them.

## 12. Dependency Policy

Before adding a new crate:

* justify it
* evaluate maintenance status
* evaluate platform support
* evaluate compile-time/runtime impact
* evaluate licensing
* evaluate security
* compare against existing repository dependencies

For native audio, explicitly compare:

* direct OS APIs
* existing abstractions
* mature cross-platform crates

Do not select a dependency merely because it is convenient.

## 13. Documentation Policy

After every issue that changes externally visible behavior, update only the relevant documentation.

At minimum, keep synchronized as applicable:

* `docs/roadmap.md`
* `docs/architecture.md`
* `docs/reactive.md`
* audio documentation
* `docs/packaging.md`
* `docs/maintain.md`
* `README.md`
* `CHANGELOG.md`

Do not rewrite unrelated documentation.

When Phase 12 progress changes, update `docs/roadmap.md` so it reflects the real milestone/issue state.

## 14. Phase 12 Definition of Done

Phase 12 is complete only when:

* native audio capture works on supported Linux, Windows and macOS configurations
* synthetic fallback remains functional
* platform backends fail gracefully
* audio data flows through the existing audio abstraction
* performance hotspots are measured and addressed where justified
* no major benchmark regressions remain
* package-manager distribution artifacts are prepared
* CLI/config behavior is stable
* configuration migration is implemented where required
* security and dependency audits pass
* full test suite passes
* CI/CD passes
* documentation is synchronized
* release candidate is verified

## 15. V1.0.0 Release Gate

Do NOT release v1.0.0 until:

* all Phase 12 milestone Issues are complete
* no P0/P1 bugs remain
* no unresolved critical review findings remain
* all supported release artifacts build successfully
* installation has been tested
* upgrade/migration paths have been tested
* checksums/provenance are valid
* performance baseline is documented
* security audit is clean or explicitly accepted
* README and docs describe the actual released behavior
* CHANGELOG is accurate
* version references are synchronized
* GitHub release is reproducible

## 16. Anti-Overengineering Rules

Do not:

* rewrite working architecture without evidence
* create unnecessary abstractions
* introduce speculative features
* add dependencies without justification
* optimize without benchmarks
* expand package targets beyond the documented scope
* refactor unrelated code
* mix multiple unrelated concerns into one issue
* silently change public behavior
* mark work complete without verification

Keep Phase 12 focused on:

`Native Audio + Measured Performance + Distribution + V1.0 Stability`

## 17. Required Planning Output

Before implementation begins, report:

1. Phase 12 audit summary
2. Milestone created
3. Final Issue list under the milestone
4. Dependency graph
5. Skill mapping per Issue using `/skill-name`
6. Execution order
7. Phase 12 Definition of Done
8. v1.0 release gates
9. Risks and blockers

Then begin implementation with the first unblocked Issue.

Do not jump ahead to later Issues while prerequisites are incomplete.

## 18. Required Per-Issue Output

After every Issue, report:

* Issue number and title
* Skills used
* Files changed
* Tests run
* Benchmark results where applicable
* Review findings
* Fixes made
* PR number/link
* CI status
* Merge status
* Remaining blockers
* Next unblocked Issue

The goal is not merely to "finish Phase 12".

The goal is to leave ZenLavaTerm in a state where **v1.0.0 is a credible, reproducible, maintainable production release**.