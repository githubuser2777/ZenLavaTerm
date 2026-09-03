# AI Test Strategy & Suite Generation Prompt

Use this prompt to design comprehensive automated tests for new or modified components in ZenLavaTerm.

---

```markdown
You are a Principal Test Architect designing automated verification suites for ZenLavaTerm (`lavaterm`).

Target Component / Subsystem:
<COMPONENT_OR_MODULE_PATH>

Test Design Requirements:
1. Unit Tests (`src/<module>/tests.rs`):
   - Test mathematical invariants (e.g. potential falloff with distance in `field.rs`, energy conservation in `physics.rs`).
   - Test boundary inputs: zero dimensions, negative coordinates, NaN or Inf floating-point values, empty buffers.
   - Test serialization/deserialization with missing or legacy fields (`config/schema.rs`).

2. Integration Tests (`tests/integration_test.rs`):
   - Test end-to-end data flow from input/signals to framebuffer rendering.
   - Test cross-platform provider contracts with `MockSystemProvider` and `MockAudioStreamFeeder`.
   - Test hardware failure and reconnection transitions (e.g. audio stream disconnect falling back to synthetic beat generator).
   - Test multi-threaded concurrency (e.g. SPSC Seqlock ring buffer under wrap-around writer contention).

3. Headless & Smoke Tests:
   - Validate CLI options via headless execution: `cargo run -- --headless --frames 30`.
   - Ensure terminal alternate screen cleanup and signal restoration (`SIGINT`, `CTRL_C_EVENT`).

4. Performance & Criterion Benchmarks (`benches/field_and_render.rs`):
   - If hot-path logic is touched, construct Criterion micro-benchmarks measuring throughput (FPS equivalent) and mean latency.

Deliverable:
- Provide the complete, compilable test code.
- Ensure all tests pass cleanly under `cargo test`.
```
