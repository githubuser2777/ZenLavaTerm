# ADR-0002: Lock-Free SPSC Seqlock Audio Ring Buffer

- **Status**: Accepted
- **Date**: 2026-08-20
- **Context**: 
  Real-time audio visualization requires capturing continuous PCM frames on a dedicated audio driver thread (CPAL worker) while the terminal rendering loop asynchronously samples recent windows for FFT analysis. Standard mutexes risk priority inversion or stuttering the audio capture thread. Conversely, naive lock-free circular buffers can tear multi-sample frames if the writer wraps around while the reader is copying.
- **Decision**:
  Implement an SPSC lock-free ring buffer (`PcmRingBuffer` in `src/audio/ring_buffer.rs`) backed by a 64-bit sequence counter (`version: AtomicU64`) and an atomic CAS spin-guard (`producer_guard`).
  - Producers increment `version` to odd before writing and to even after finishing.
  - Readers check `version` before and after copying. If contention or a wrap-around occurs, readers retry up to 64 times.
  - If sustained contention occurs, readers return `false` and clear the buffer rather than returning torn or unverified data, delegating smoothly to synthetic fallback in `LiveAudioProvider`.
- **Consequences**:
  - **Positive**: Readers never block producers; guaranteed tear-free window snapshots for FFT; zero priority inversion in the audio driver callback.
  - **Negative / Trade-offs**: Custom lock-free concurrency requires careful memory ordering (`Acquire`, `Release`, `Relaxed`) and dedicated multi-threaded contention integration tests.
  - **Invariants**: Readers must never block producers; partial frame tearing must be rejected.
