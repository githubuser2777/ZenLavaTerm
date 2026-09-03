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

- **SPSC Lock-Free Seqlock Ring Buffer (`PcmRingBuffer`)**:
  - **Concurrency Contract**: Operates fundamentally on a Single-Producer, Single-Consumer (SPSC) model where the CPAL audio capture thread writes frames and the terminal visualization thread reads recent analysis windows.
  - **Tear-Free Snapshot Consistency**: Employs a 64-bit sequence counter (`version`). The producer transitions `version` to odd before writing and even after writing. Consumer reads check `version` before and after copying; if a wrap-around or write collision occurs during read, the reader retries. This mathematically guarantees that the FFT window never mixes older and newly overwritten sample generations.
  - **Multi-Producer Hardening**: Uses a low-overhead atomic CAS spin-guard (`producer_guard`) to prevent index clobbering if multiple producer threads write simultaneously.
  - **Coherent Buffer Reset**: `clear()` is coordinated under the version sequence lock, ensuring readers never observe partially-cleared states.
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

