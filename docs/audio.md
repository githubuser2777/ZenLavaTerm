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

### Runtime Providers

- **Windows WASAPI Capture (`WindowsAudioCapture`)**: Captures default render loopback (speakers/headphones output mix) and microphone streams via Win32 MMDevice / WASAPI.
- **Linux PipeWire / ALSA Capture (`LinuxAudioCapture`)**: Captures live PCM streams from default PulseAudio/PipeWire sources or ALSA hardware devices (`hw:0,0`).
- **macOS CoreAudio Capture (`MacOSAudioCapture`)**: Captures default input device streams via CoreAudio HAL with graceful permission handling.
- **Synthetic Fallback (`SyntheticAudioGenerator`)**: Procedurally generates rhythmic harmonic beat pulses at configurable `bpm` whenever audio hardware is absent, disabled, or permissions are denied.

## Frequency Band Mappings

| Frequency Band | Range | Lava Physical Effect |
|---|:---:|---|
| **Bass** | $20\text{ Hz} - 250\text{ Hz}$ | Gives powerful upward convective thrust ($0.80 + 1.50 \times \text{bass}$) mimicking bass kicks. |
| **Midrange** | $250\text{ Hz} - 4,000\text{ Hz}$ | Modulates Brownian fluid turbulence ($0.15 \times (1.0 + 2.5 \times \text{mid})$). |
| **Treble** | $4,000\text{ Hz} - 20,000\text{ Hz}$ | Imparts subtle kinetic perturbation and surface agitation. |

## Usage

### Enabling via CLI Flag

```bash
# List all available audio capture devices
lavaterm --list-audio-devices

# Run LavaTerm with default audio capture reactivity
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

