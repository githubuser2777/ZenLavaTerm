# Audio-Reactive Mode

LavaTerm features a real-time audio visualization engine that responds to musical rhythms and ambient acoustics in the terminal.

## Architecture

The audio pipeline follows a decoupled producer-consumer model:

```text
┌─────────────────────────────────────────────────────────────┐
│                 Audio Capture Providers                     │
│  - SyntheticAudioGenerator (procedural beat generator)      │
│  - LiveAudioProvider (PCM sample ring buffer stream)        │
│  - MockAudioProvider (deterministic unit/integration tests) │
└──────────────────────────────┬──────────────────────────────┘
                               │ Ingests PCM samples
                               ▼
┌─────────────────────────────────────────────────────────────┐
│             SPSC Lock-Free PCM Ring Buffer (Seqlock)        │
│  - Atomic circular buffer with 64-bit sequence lock (Seqlock)│
│  - Tear-free snapshot consistency under wrap-around         │
│  - Multi-producer guard and non-blocking real-time reads    │
└──────────────────────────────┬──────────────────────────────┘
                               │ Extracts analysis window
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Spectrum Analyzer & FFT                     │
│  - Hann windowing function                                  │
│  - Cooley-Tukey Radix-2 in-place FFT                        │
│  - Band energy integration into [0.0, 1.0]                  │
└──────────────────────────────┬──────────────────────────────┘
                               │ Produces AudioSignals
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Simulation Core (Blobs)                     │
│  - Bass   (20-250 Hz)    ──> Convective buoyancy surge      │
│  - Mid    (250-4000 Hz)  ──> Fluid turbulence & noise       │
│  - Treble (4000-20k Hz)  ──> Micro-perturbation jitter      │
└─────────────────────────────────────────────────────────────┘
```

### Runtime Providers, SPSC Concurrency & Fallback

- **SPSC Ring Buffer with Lock-Free Reader, Serialized Multi-Producer Guard, and Seqlock (`PcmRingBuffer`)**:
  - **Concurrency Contract**: Operates fundamentally on a Single-Producer, Single-Consumer (SPSC) model where the CPAL audio capture thread writes frames and the terminal visualization thread reads recent analysis windows.
  - **Strict Tear-Free Snapshot Consistency**: Employs a 64-bit sequence counter (`version`). The producer transitions `version` to odd before writing and even after writing. Consumer reads verify `version` before and after copying. If contention or a wrap-around occurs during read, `read_recent` retries up to 64 times. If sustained contention prevents verification, it strictly refuses to return unverified or torn data (returns `false` and clears the buffer). `LiveAudioProvider` gracefully delegates to `SyntheticAudioGenerator` if a coherent read is not obtained, guaranteeing FFT never receives torn frames.
  - **Serialized Multi-Producer Guard**: Uses a low-overhead atomic CAS spin-guard (`producer_guard`) to serialize writes if multiple producer threads write simultaneously, preventing index corruption while maintaining zero overhead for single producers.
  - **Coherent Buffer Reset**: `clear()` is coordinated under the version sequence lock and producer guard, ensuring readers never observe partially-cleared states.
- **Native Stream Capture (`NativeAudioCapture`)**: Background streaming capture worker powered by `cpal`, providing cross-platform hardware audio capture across Linux (ALSA), Windows (WASAPI), and macOS (CoreAudio). It detects default or user-specified audio endpoints and captures raw PCM streams into the ring buffer.
- **Runtime Stream Fallback & Live Resumption**: `NativeAudioCapture` and `LiveAudioProvider` share an atomic `stream_alive: Arc<AtomicBool>` flag. If the active hardware stream disconnects (e.g. unplugging headphones/DAC or driver crash), CPAL error callbacks immediately trip `stream_alive` to false. `LiveAudioProvider::poll_signals()` automatically falls back to an internal `SyntheticAudioGenerator(bpm)` instance, ensuring the terminal lava visualizer never freezes or flatlines into silence. When the audio backend stream is restored/reconnected, live signal processing resumes automatically.
- **Synthetic Generator (`SyntheticAudioGenerator`)**: Procedurally generates rhythmic harmonic beat pulses at configurable `bpm` whenever audio capture is disabled, or when native hardware capture is unavailable or disconnected.
- **Hardware Frame Simulator (`MockAudioStreamFeeder`)**: Simulates continuous real hardware audio frame streams (f32, i16, u16) with background threads, hardware disconnect/reconnect simulation, and buffer overrun/underrun testing.
- **Spectrum Analyzer (`SpectrumAnalyzer`)**: Implements an in-place Cooley-Tukey Radix-2 FFT with Hann windowing and spectral band integration, configured dynamically to match the active capture device's sample rate.
- **Sample Rate Converter (`resample_linear`)**: Linear interpolation resampler utility in `PcmRingBuffer` for sample rate conversions (e.g. 48,000 Hz <-> 44,100 Hz).


## Frequency Band Mappings

| Frequency Band | Range | Lava Physical Effect |
|---|:---:|---|
| **Bass** | $20\text{ Hz} - 250\text{ Hz}$ | Gives powerful upward convective thrust ($0.80 + 1.50 \times \text{bass}$) mimicking bass kicks. |
| **Midrange** | $250\text{ Hz} - 4,000\text{ Hz}$ | Modulates Brownian fluid turbulence ($0.15 \times (1.0 + 2.5 \times \text{mid})$). |
| **Treble** | $4,000\text{ Hz} - 20,000\text{ Hz}$ | Imparts subtle kinetic perturbation and surface agitation. |

## Usage

### Enabling via CLI Flag

```bash
# List available audio capture devices
lavaterm --list-audio-devices

# Run LavaTerm with audio-reactive mode
lavaterm --audio

# Select a specific audio input/output device
lavaterm --audio --audio-device "default"

# Combine with Braille renderer and custom framerate
lavaterm --audio --renderer braille --fps 60
```

### Enabling via TOML Configuration

```toml
[audio]
enabled = true
bpm = 120.0
device = "default"  # Optional specific device name
```

---

## Platform Specifics & Operational Caveats

### 1. Windows WASAPI Loopback (`--audio-loopback`)
- **Mechanism**: On Windows, ZenLavaTerm connects to the default audio render endpoint (`eRender`) via WASAPI. CPAL automatically sets `AUDCLNT_STREAMFLAGS_LOOPBACK` on the audio client, enabling capture of the system's shared output mix (e.g., music players, browser playback, games).
- **Silence & Buffer Event Behavior**: Under Windows WASAPI architecture, the core audio engine only dispatches loopback buffer packets when audio is actively rendering through the selected output endpoint. If no sound is currently playing, WASAPI suppresses buffer events or produces silent underruns. Consequently, ZenLavaTerm's visualizer remains quiescent or smoothly delegates to synthetic beats until audio begins playing.
- **Headless CI Verification Limitation**: Automated CI runners (GitHub Actions `windows-latest`) operate in headless virtual machines without physical audio hardware or active audio render endpoints. CI verifies the device enumeration contract, configuration schema, error handling, and in-memory mock frame pipelines (`MockAudioStreamFeeder`), but live loopback PCM capture must be verified on physical Windows hardware with active audio output.

### 2. Live Audio Streaming vs. Resampling
- **Dynamic Hardware Sampling Rate**: To eliminate resampling overhead, phase distortion, and chunk boundary discontinuities, `LiveAudioProvider` queries the capture device's native sample rate (e.g., 44.1 kHz, 48 kHz, 96 kHz) upon stream creation and initializes `SpectrumAnalyzer` with that exact hardware rate.
- **Zero Boundary Discontinuity in Live Path**: Because the FFT spectrum analyzer adapts directly to the hardware sample rate, the live capture callback ingests raw PCM frames directly into the ring buffer without per-chunk resampling.
- **Resampling Utility**: `resample_linear` and `PcmRingBuffer::push_resampled` provide lightweight linear interpolation for testing, format normalization, or synthetic pipelines.

