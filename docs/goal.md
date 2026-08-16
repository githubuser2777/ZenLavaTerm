# LavaTerm — Project Goal

## 1. North Star

LavaTerm is a terminal-native ambient visualization engine whose first visualizer is a living lava lamp.

The core experience is:

```text
Open terminal
    ↓
See beautiful metaball lava
    ↓
Lava moves naturally
    ↓
Lava can react to signals from the machine
```

The project should feel like a real native terminal utility, not a one-off ASCII demo.

Primary product goal:

> Build a beautiful, smooth, lightweight, configurable terminal ambient visualizer with a clean architecture that can grow beyond lava.

## 2. Product Identity

LavaTerm sits between:

```text
Terminal eye-candy
        +
System observability
        +
Interactive simulation
        +
Ricing / customization
```

The key concept is **ambient observability**:

System state does not have to be presented only as numbers. It can be represented as motion, shape, color and energy.

Example:

```text
CPU      → turbulence
RAM      → blob size
Audio    → movement
Battery  → temperature
Network  → drift
Time     → palette
Input    → ripple
```

## 3. Core Architecture Goal

```text
Signals / Inputs
      ↓
Reactive State
      ↓
Lava Simulation
      ↓
Virtual Framebuffer
      ↓
Renderer
      ↓
Terminal Backend
```

The most important architectural rule:

> The simulation core must not know that a terminal exists.

Core responsibilities:

```text
physics
metaballs
scalar fields
simulation state
time
signals
```

Renderer responsibilities:

```text
framebuffer → terminal representation
```

Platform/backend responsibilities:

```text
audio
system metrics
terminal I/O
platform APIs
```

## 4. MVP Goal

The MVP must provide:

- metaball lava simulation;
- basic believable physics;
- virtual framebuffer;
- terminal rendering;
- half-block renderer;
- true-color gradient;
- stable animation loop;
- TOML configuration;
- graceful terminal cleanup;
- deterministic tests for core logic.

Running:

```bash
lavaterm
```

must immediately produce a usable visual experience without requiring a configuration file.

The MVP does NOT include:

- audio reactive mode;
- system metrics;
- PipeWire;
- mouse interaction;
- tmux/zellij integration;
- advanced fluid simulation;
- automatic theme detection.

## 5. Long-Term Product Goal

After the MVP, LavaTerm should evolve into an ambient visualization toolkit.

Potential visualizers:

```text
lava
plasma
smoke
fire
liquid
aurora
```

The engine should eventually support multiple visual behaviors using the same:

```text
signals
→ simulation
→ framebuffer
→ renderer
```

pipeline.

Lava is the first visualization.

## 6. Simulation Goal

Use metaballs and lightweight procedural physics.

Preferred model:

```text
Blob
├── position
├── velocity
├── radius
├── temperature
└── state / phase
```

Possible forces:

```text
gravity
buoyancy
viscosity
turbulence
noise
```

Prioritize:

```text
visual quality
+
stability
+
predictability
```

over physical accuracy.

Do NOT attempt a full fluid solver unless a future requirement explicitly justifies it.

## 7. Rendering Goal

Rendering should use a virtual framebuffer so simulation resolution is independent from terminal resolution.

Target renderer family:

```text
HalfBlock
Block
Braille
ASCII
```

The renderer must be replaceable without changing simulation logic.

Output should be efficiently batched.

Avoid:

```text
print() per cell
```

Prefer:

```text
framebuffer
→ compare with previous frame
→ build ANSI output
→ one/batched writes
```

## 8. Visual Goal

Lava should have:

- smooth movement;
- organic blob merging;
- natural rising/falling;
- coherent color gradients;
- responsive but not chaotic animation;
- good appearance at multiple terminal sizes.

Temperature-based color should use configurable palette interpolation.

Example:

```text
cold
  ↓
violet
  ↓
purple
  ↓
orange
  ↓
red
  ↓
hot
```

Do not hard-code a single visual theme into the renderer.

## 9. Configuration Goal

Use TOML for user customization.

Configuration should eventually cover:

```text
simulation
rendering
palette
audio
system
theme
```

Keep the initial schema small.

Example:

```toml
[simulation]
blobs = 12
gravity = 0.12
buoyancy = 0.8
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

Defaults must always produce a reasonable experience.

## 10. Reactive Systems Goal

Future data sources should be adapters, not entangled with simulation.

```text
Audio Provider
System Provider
Input Provider
Time Provider
        ↓
Reactive Signals
        ↓
Simulation
```

Possible signals:

```text
audio.bass
audio.mid
audio.treble

system.cpu
system.memory
system.battery
system.network
system.disk

input.keyboard
input.mouse

time.hour
time.minute
```

Simulation should consume normalized signals rather than know their original source.

## 11. Audio Goal

Eventually support:

```text
PCM
→ FFT
→ bass / mid / treble
→ reactive signals
→ lava behavior
```

Linux priority:

```text
PipeWire
```

Future platform backends:

```text
Windows → WASAPI
macOS   → CoreAudio
```

Audio must not block the render loop.

If audio is unavailable, LavaTerm should fall back gracefully to normal lava simulation.

## 12. System Reactive Goal

Eventually visualize:

```text
CPU
RAM
Battery
Disk I/O
Network
```

Potential mappings:

```text
CPU      → turbulence
RAM      → blob size
Disk I/O → bubble activity
Network  → drift
Battery  → temperature
```

System monitoring is an enhancement, not MVP scope.

## 13. Theme / Ricing Goal

LavaTerm should fit terminal ricing environments.

Potential sources:

```text
pywal
wallust
ANSI terminal palette
Base16-style palettes
```

Future command:

```bash
lavaterm --theme auto
```

A profile may eventually control:

```text
palette
viscosity
noise
gravity
blob size
reactivity
```

Therefore:

> Theme can eventually mean appearance + behavior.

## 14. Terminal Ecosystem Goal

LavaTerm should work well inside:

```text
tmux
zellij
```

Potential modes:

```bash
lavaterm --compact
lavaterm --widget
```

Widget mode must support:

- adaptive dimensions;
- resize;
- low output noise;
- stable FPS;
- graceful cleanup.

## 15. Interaction Goal

Later versions may support:

```text
mouse click → impact
mouse drag  → stir
keyboard    → ripple
scroll      → pressure
```

Interaction comes after simulation and renderer quality are stable.

## 16. Cross-Platform Goal

Primary development target:

```text
Linux
```

Architecture should remain portable enough for:

```text
Linux
Windows
macOS
```

Platform-specific APIs belong behind backend abstractions.

Do not scatter OS-specific behavior through core simulation code.

## 17. Quality Goals

LavaTerm should be:

```text
lightweight
fast
stable
memory-safe
testable
configurable
portable
```

Quality gates:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

Performance workflow:

```text
measure
→ identify bottleneck
→ optimize
→ measure again
```

## 18. Testing Goal

Core behavior should be testable without a terminal.

Test areas:

```text
blob initialization
field evaluation
thresholding
physics
timestep handling
noise determinism
color interpolation
configuration validation
signal mapping
FFT
```

Use deterministic inputs whenever practical.

## 19. Release Strategy

Development is organized by phases. Each phase is a milestone completed before moving to the next.

Recommended progression:

```text
Phase 0 → v0.1.0
Project foundation

Phase 1 → v0.2.0
Simulation core

Phase 2 → v0.3.0
Virtual framebuffer

Phase 3 → v0.4.0
Terminal renderer

Phase 4 → v0.5.0
Additional renderers

Phase 5 → v0.6.0
Configuration

Phase 6 → v0.7.0
System reactive

Phase 7 → v0.8.0
Audio reactive

Phase 8 → v0.9.0
Theme engine

Phase 9 → v0.10.0
tmux / zellij

Phase 10 → v0.11.0
Interaction

Phase 11 → later
Cross-platform expansion

Phase 12 → v1.0.0
Stable release
```

These versions are milestone markers, not a strict semantic-version promise.

## 20. Phase Completion Rule

A phase is complete only when:

```text
All planned issues complete
        ↓
Acceptance criteria satisfied
        ↓
Tests pass
        ↓
Lint passes
        ↓
Build passes
        ↓
Documentation updated
        ↓
Git history clean
        ↓
Release/tag created
```

Only then should the next phase begin.

The AI must not silently continue into the next phase.

## 21. AI Coding Agent Rules

The AI must:

- inspect the repository before changing it;
- read relevant documentation;
- work only on the current phase;
- respect issue boundaries;
- keep changes atomic;
- test behavior;
- validate before claiming completion;
- update documentation when behavior changes;
- commit logically;
- report limitations honestly.

The AI must not:

- implement future phases "while already here";
- add unrelated features;
- introduce dependencies without justification;
- replace architecture with shortcuts;
- create unnecessary abstractions;
- optimize without evidence;
- claim tests passed without running them;
- close an issue whose acceptance criteria are not satisfied.

## 22. Scope Control

When a new idea appears during implementation:

```text
Required for current issue
    → implement now

Useful but not required
    → create future issue

Unclear
    → document as open question

Out of product scope
    → do not implement
```

The current phase always wins over speculative improvements.

## 23. Definition of Success

LavaTerm is successful when a user can:

```text
install LavaTerm
      ↓
run `lavaterm`
      ↓
immediately see beautiful lava
      ↓
customize it through config
      ↓
trust it to run continuously
      ↓
optionally connect system/audio/theme signals
```

And a developer can:

```text
read the architecture
      ↓
understand the simulation
      ↓
add a renderer/provider
      ↓
write tests
      ↓
ship a change without breaking the core
```

## 24. North Star Reminder

When making a design decision, ask:

> Does this make LavaTerm a better terminal-native ambient visualization engine without unnecessarily increasing complexity?

If yes, continue.

If it is merely technically interesting, defer it.

If it breaks the core architecture, redesign it.

If it belongs to a later phase, create an issue and keep the current phase focused.
