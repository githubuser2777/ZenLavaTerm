# Benchmarking & Performance Profiling

ZenLavaTerm maintains a comprehensive micro-benchmark suite in `benches/field_and_render.rs` using Criterion to prevent performance regressions in critical inner loops.

---

## 1. Running Benchmarks

```bash
# Build benchmarks without executing (fast compiler check)
cargo bench --no-run

# Run full Criterion benchmark suite
cargo bench

# Run specific benchmark group (e.g. rasterization)
cargo bench --bench field_and_render -- rasterization
```

---

## 2. Empirical Performance Baselines

The detailed empirical benchmark report is recorded in [docs/benchmarks/benchmark_baseline.md](file:///home/skids/Documents/code/ZenLavaTerm/docs/benchmarks/benchmark_baseline.md).

### Key Performance Metrics:
1. **Scalar Field Evaluation**:
   - $80 \times 20$ grid evaluation across 6, 12, and 24 blobs executes in **~423 ns** (~3.77 million evaluations per second).
   - Invariant hoisting and SIMD-friendly loops ensure near $O(1)$ scaling up to 24 blobs.
2. **Framebuffer Rasterization**:
   - Standard terminal dimension ($80 \times 48$) smooth gradient executes in **127.14 µs** (~**7,865 FPS** throughput).
   - Stepped gradient executes in **62.68 µs** (~**15,954 FPS** throughput, a **50.7% speedup**).
   - High resolution ($120 \times 60$) executes in **236.65 µs** (~**4,225 FPS** throughput).
3. **Terminal Renderers ($80 \times 48$)**:
   - Half-Block: **83.85 µs** (~11,926 FPS)
   - Block: **81.24 µs** (~12,309 FPS)
   - Braille ($160 \times 96$ subpixels): **72.33 µs** (~13,825 FPS)

---

## 3. Profiling Inner Loops

When investigating performance bottlenecks:
- Ensure no allocations occur within `rasterize_simulation` or `Renderer::render`.
- Use `perf` on Linux or Instruments on macOS:
  ```bash
  cargo build --release
  perf record --call-graph dwarf ./target/release/lavaterm --headless --frames 3000
  perf report
  ```
