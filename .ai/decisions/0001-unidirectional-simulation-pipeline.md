# ADR-0001: Unidirectional Simulation & Rendering Pipeline

- **Status**: Accepted
- **Date**: 2026-08-10
- **Context**: 
  Metaball fluid visualizers can easily devolve into tightly coupled spaghetti where rendering logic interacts directly with physics state, or platform event loops mutate simulation fields directly. We need strict modularity so simulation math can be benchmarked and tested independently of terminal escape sequences, and renderers can be swapped or tested in headless environments.
- **Decision**:
  Enforce a strict unidirectional pipeline:
  `Platform / Signals -> Simulation Core -> Virtual Framebuffer -> Terminal Renderer -> Stdout TTY`.
  `src/core/` operates strictly in continuous normalized space `[0.0, 1.0]` and has zero knowledge of `crossterm`, terminal cells, or operating systems. Renderers consume a read-only borrow of `VirtualFramebuffer` and emit batched ANSI byte streams to `stdout`.
- **Consequences**:
  - **Positive**: 100% deterministic unit testing of physics and field calculations; zero-cost headless execution (`--headless`); independent benchmarking of rasterization loops vs rendering loops.
  - **Negative / Trade-offs**: Requires intermediate translation step mapping discrete terminal coordinates to continuous normalized coordinates and rasterizing field evaluations into an offscreen framebuffer before ANSI emission.
  - **Invariants**: `src/core/` must never import `crossterm` or platform telemetry crates.
