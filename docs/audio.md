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
│                 Lockless PCM Ring Buffer                    │
│  - Decouples high-rate audio capture from render loop       │
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

### Runtime Providers vs. Implemented Infrastructure

- **Active Default Provider (`--audio`)**: Uses `SyntheticAudioGenerator` to procedurally simulate rhythmic beat pulses at a configurable tempo (`bpm`), enabling fluid audio-reactive kinematics out-of-the-box in headless and interactive environments without requiring hardware audio daemons or root audio permissions.
- **Spectrum Analysis Infrastructure**: `SpectrumAnalyzer` implements in-place Cooley-Tukey Radix-2 FFT with Hann windowing and spectral energy isolation.
- **Buffer & Stream Infrastructure**: `PcmRingBuffer` and `LiveAudioProvider` implement thread-safe circular buffering and FFT analysis ready for external PCM sample stream ingestion.

## Frequency Band Mappings

| Frequency Band | Range | Lava Physical Effect |
|---|:---:|---|
| **Bass** | $20\text{ Hz} - 250\text{ Hz}$ | Gives powerful upward convective thrust ($0.80 + 1.50 \times \text{bass}$) mimicking bass kicks. |
| **Midrange** | $250\text{ Hz} - 4,000\text{ Hz}$ | Modulates Brownian fluid turbulence ($0.15 \times (1.0 + 2.5 \times \text{mid})$). |
| **Treble** | $4,000\text{ Hz} - 20,000\text{ Hz}$ | Imparts subtle kinetic perturbation and surface agitation. |

## Usage

### Enabling via CLI Flag

```bash
# Run LavaTerm with audio reactivity enabled
lavaterm --audio

# Combine with Braille renderer and custom framerate
lavaterm --audio --renderer braille --fps 60
```

### Enabling via TOML Configuration

```toml
[audio]
enabled = true
bpm = 120.0
```
