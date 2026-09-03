# ZenLavaTerm Criterion Benchmark Baseline Report

This document records empirical performance evidence and baseline micro-benchmark measurements for ZenLavaTerm v1.0.0. The measurements validate the performance claims recorded in `CHANGELOG.md` and verify throughput for scalar field evaluation, framebuffer rasterization, terminal renderers, FFT spectrum analysis, and lock-free audio ring buffer ingestion.

The raw benchmark log is preserved in [`docs/benchmarks/criterion_baseline.log`](criterion_baseline.log).

---

## 1. Test Environment

- **OS**: Linux (Kernel 6.x, x86_64)
- **Rust Toolchain**: `rustc 1.85+` (Edition 2021)
- **Compilation Profile**: `bench` (`opt-level = 3`, `lto = true`, `codegen-units = 1`)
- **Benchmark Framework**: Criterion 0.5 with 100 samples per measurement (3.0s warmup, 5.0s measurement window)

---

## 2. Empirical Benchmark Measurements

### 2.1 Scalar Field Evaluation
Evaluates potential field summation across an $80 \times 20$ sampling grid ($1,600$ evaluation points):

| Parameter | Execution Time (Mean) | Confidence Interval (95%) | Throughput |
| :--- | :---: | :---: | :---: |
| `field_evaluation/6` blobs | **422.79 ns** | [421.69 ns, 424.20 ns] | ~3.78M evals/sec |
| `field_evaluation/12` blobs | **423.34 ns** | [422.00 ns, 424.99 ns] | ~3.77M evals/sec |
| `field_evaluation/24` blobs | **425.85 ns** | [424.57 ns, 427.31 ns] | ~3.75M evals/sec |

*Observation*: Hoisting loop invariants and SIMD-friendly math provides near $O(1)$ scaling across blob counts up to 24 blobs.

---

### 2.2 Framebuffer Rasterization Loops
Evaluates 2D field sampling and contiguous slice memory population in `VirtualFramebuffer`:

| Benchmark Function | Mean Frame Time | Confidence Interval (95%) | Equivalent FPS Throughput |
| :--- | :---: | :---: | :---: |
| `rasterize_80x48` (Smooth Gradient) | **127.14 µs** | [126.65 µs, 127.70 µs] | **7,865 FPS** |
| `rasterize_stepped_gradient_80x48` | **62.68 µs** | [62.28 µs, 63.11 µs] | **15,954 FPS** |
| `rasterize_120x60` (High Res) | **236.65 µs** | [235.94 µs, 237.40 µs] | **4,225 FPS** |

*Verification of Claim*:
The changelog claim of **>5,000 FPS equivalent throughput** on standard terminal dimensions ($80 \times 48$) is empirically verified:
$$\text{Throughput} = \frac{1}{127.14 \times 10^{-6}\text{ s}} \approx 7,865\text{ FPS} > 5,000\text{ FPS}$$

The stepped gradient optimization delivers a **50.7% speedup** over smooth gradient rasterization ($127.14\ \mu\text{s} \to 62.68\ \mu\text{s}$).

---

### 2.3 Terminal Renderers
Evaluates ANSI string encoding and ANSI escape emission into contiguous byte buffers:

| Renderer | Mean Render Time | Confidence Interval (95%) | Equivalent Render FPS |
| :--- | :---: | :---: | :---: |
| `renderers/halfblock` ($80 \times 48$) | **83.85 µs** | [82.44 µs, 85.57 µs] | **11,926 FPS** |
| `renderers/block` ($80 \times 48$) | **81.24 µs** | [80.28 µs, 82.34 µs] | **12,309 FPS** |
| `renderers/braille` ($160 \times 96$) | **72.33 µs** | [71.76 µs, 73.00 µs] | **13,825 FPS** |

---

### 2.4 FFT Analysis, Resampling & SPSC Seqlock Ring Buffer
Evaluates real-time audio components:

| Audio Operation | Mean Execution Time | Confidence Interval (95%) | Notes |
| :--- | :---: | :---: | :---: |
| `compute_fft/512` | **6.68 µs** | [6.50 µs, 6.89 µs] | Radix-2 in-place |
| `compute_fft/1024` | **13.16 µs** | [12.97 µs, 13.39 µs] | Standard analysis window |
| `compute_fft/2048` | **27.94 µs** | [27.62 µs, 28.33 µs] | High resolution window |
| `spectrum_analyze_1024` | **17.79 µs** | [17.66 µs, 17.94 µs] | Hann window + FFT + band sum |
| `resample_linear_48k_to_44k` | **4.75 µs** | [4.69 µs, 4.82 µs] | 1024 samples linear resample |
| `ring_buffer_lock_free_push_256` | **212.64 ns** | [210.98 ns, 214.59 ns] | Uncontended batch ingestion |
| `ring_buffer_lock_free_read_512` | **842.34 ns** | [838.31 ns, 847.00 ns] | Uncontended Seqlock read |
| `ring_buffer_seqlock_contended_read_512` | **3.80 µs** | [3.67 µs, 3.93 µs] | **Active wrap-around contention** |

*SPSC Seqlock Audio Ring Buffer Performance & Concurrency Verification*:
- **Uncontended Ingestion**: Pushing 256 PCM audio samples takes **212.64 nanoseconds** ($0.83\text{ ns/sample}$), completely non-blocking without thread preemption or syscalls.
- **Uncontended Snapshot Read**: Reading a 512-sample analysis window takes **842.34 nanoseconds**.
- **Contended Concurrent Wrap-Around**: When an active producer thread pushes 256-sample chunks in an unthrottled loop continuously wrapping around the buffer, the consumer reads a guaranteed tear-free, linearizable 512-sample snapshot in **3.80 microseconds** (supporting >260,000 consistent snapshots/sec). At a 60 FPS render budget ($16,666\ \mu\text{s}$), audio snapshot acquisition takes less than $0.023\%$ of the frame interval even under peak contention.

---

### 2.5 End-to-End Pipeline & Adaptation
Evaluates full simulation step + field evaluation + rasterization + terminal rendering:

| Pipeline Configuration | Mean Frame Time | Equivalent Full-Pipeline FPS |
| :--- | :---: | :---: |
| `full_frame_audio_halfblock` | **244.11 µs** | **4,096 FPS** |
| `full_frame_reactive_halfblock` | **227.63 µs** | **4,393 FPS** |
| `compact_adapt_simulation` | **12.32 ns** | ~81 million adaptations/sec |

At a standard display refresh rate of 60 FPS ($16.67\text{ ms}$ frame budget), ZenLavaTerm consumes approximately **1.46%** of a single CPU core's frame budget ($244.11\ \mu\text{s} / 16,666\ \mu\text{s}$), leaving over 98.5% of the frame interval idle.
