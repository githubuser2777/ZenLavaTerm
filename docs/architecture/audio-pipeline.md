# Audio Architecture & Signal Processing

ZenLavaTerm includes a real-time audio visualization pipeline that transforms live PCM microphone input, loopback audio, or procedural synthetic beats into fluid kinetic responses.

---

## 1. Audio Processing Pipeline

```text
┌─────────────────────────────────────────────────────────────┐
│                 Audio Capture Providers                     │
│  - NativeAudioCapture (CPAL: ALSA / WASAPI / CoreAudio)     │
│  - SyntheticAudioGenerator (procedural harmonic beat)       │
│  - MockAudioProvider (deterministic test streams)           │
└──────────────────────────────┬──────────────────────────────┘
                               │ Raw PCM Samples (f32)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│             SPSC Lock-Free PCM Ring Buffer (Seqlock)        │
│  - 4096-sample circular buffer                              │
│  - 64-bit sequence counter (AtomicU64)                      │
│  - Atomic CAS producer spin-guard                           │
│  - Up to 64 non-blocking retries on writer wrap-around      │
└──────────────────────────────┬──────────────────────────────┘
                               │ Window snapshot (1024 samples)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Spectrum Analyzer & FFT                     │
│  - Dynamic hardware sample rate configuration (e.g. 48 kHz) │
│  - Hann windowing function                                  │
│  - In-place Cooley-Tukey Radix-2 FFT                        │
│  - Band energy integration into [0.0, 1.0]                  │
└──────────────────────────────┬──────────────────────────────┘
                               │ AudioSignals { bass, mid, treble }
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Simulation Fluid Response                   │
│  - Bass   (20 - 250 Hz)   ──> Convective buoyancy surges    │
│  - Mid    (250 - 4000 Hz) ──> Fluid turbulence & noise      │
│  - Treble (4000 - 20k Hz) ──> Kinetic surface jitter        │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Lock-Free SPSC Seqlock Buffer (`src/audio/ring_buffer.rs`)

To eliminate audio stutter and prevent mutex priority inversion:

1. **Concurrency Contract**: Single-Producer (CPAL audio thread) Single-Consumer (visualization loop thread).
2. **Seqlock Sequence Counter**:
   - The writer increments `version` to an odd number before updating samples, and increments to an even number after completion.
   - The reader inspects `version` before and after copying data. If `version` changed or was odd, a collision occurred, and the reader retries up to 64 times.
3. **Fail-Safe Fallback**:
   - If sustained contention prevents verification after 64 retries, the reader returns `false` and clears the output buffer rather than returning unverified, torn data.
   - `LiveAudioProvider` gracefully delegates to `SyntheticAudioGenerator` during contention, ensuring the FFT never processes corrupt frames.
4. **Producer Guard**:
   - An atomic CAS flag (`producer_guard`) serializes writes if multiple threads attempt to push simultaneously, maintaining thread safety without locking the reader.

---

## 3. FFT Spectrum Analysis (`src/audio/fft.rs`)

- **Dynamic Hardware Sampling**: Adapts dynamically to the capture device's native hardware sample rate (e.g., 44.1 kHz, 48 kHz, 96 kHz) at stream initialization, eliminating per-chunk resampling overhead.
- **Windowing**: Applies a Hann window to smooth boundary discontinuities prior to the Fourier Transform:
  $$w(n) = 0.5 \left(1 - \cos\left(\frac{2\pi n}{N - 1}\right)\right)$$
- **Radix-2 Cooley-Tukey**: Computes complex frequency magnitudes in-place over power-of-two window sizes (default $N = 1024$).
- **Spectral Energy Bands**:
  - **Bass** ($20\text{ Hz} - 250\text{ Hz}$): Maps to thermal convection and buoyant lift.
  - **Mid** ($250\text{ Hz} - 4,000\text{ Hz}$): Maps to Brownian fluid perturbation.
  - **Treble** ($4,000\text{ Hz} - 20,000\text{ Hz}$): Maps to micro-jitter kinetic noise.

---

## 4. Hardware Disconnect Resilience & Fallback

- Native stream capture (`NativeAudioCapture`) and `LiveAudioProvider` share an atomic `stream_alive: Arc<AtomicBool>` flag.
- If hardware is disconnected (e.g., unplugging USB DAC/headphones or device reset), CPAL error callbacks set `stream_alive` to `false`.
- `LiveAudioProvider::poll_signals()` seamlessly delegates to an internal `SyntheticAudioGenerator`, maintaining smooth visualizer animation. When the device is restored, live processing resumes automatically.
