//! Audio provider interfaces, mock fixtures, and synthetic audio generators.

use super::signals::AudioSignals;

/// Trait implemented by audio capture providers (PipeWire, WASAPI, CoreAudio, mock, synthetic).
pub trait AudioProvider: Send {
    /// Polls latest normalized frequency signals.
    fn poll_signals(&mut self) -> AudioSignals;

    /// Returns true if this provider is consuming from a live hardware audio stream.
    fn is_live(&self) -> bool {
        false
    }

    /// Returns the provider backend name (e.g. "synthetic", "mock", "live", "wasapi", "pipewire", "coreaudio").
    fn provider_name(&self) -> &'static str {
        "unknown"
    }
}

/// Metadata description for an enumerated audio input or loopback device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    /// Device identifier or name.
    pub name: String,
    /// Whether this is the system default audio capture/render device.
    pub is_default: bool,
}

/// Deterministic mock audio provider for unit testing.
#[derive(Debug, Clone)]
pub struct MockAudioProvider {
    pub signals: AudioSignals,
}

impl MockAudioProvider {
    /// Creates a new `MockAudioProvider` with fixed signals.
    pub fn new(signals: AudioSignals) -> Self {
        Self { signals }
    }
}

impl AudioProvider for MockAudioProvider {
    fn poll_signals(&mut self) -> AudioSignals {
        self.signals
    }

    fn is_live(&self) -> bool {
        false
    }

    fn provider_name(&self) -> &'static str {
        "mock"
    }
}

/// Procedural synthetic audio signal generator for headless benchmarking and demos.
#[derive(Debug, Clone)]
pub struct SyntheticAudioGenerator {
    /// Internal elapsed time counter.
    pub time: f32,
    /// Beats per minute rhythm parameter.
    pub bpm: f32,
}

impl Default for SyntheticAudioGenerator {
    fn default() -> Self {
        Self::new(120.0)
    }
}

impl SyntheticAudioGenerator {
    /// Creates a new synthetic audio generator at specified BPM.
    pub fn new(bpm: f32) -> Self {
        Self { time: 0.0, bpm }
    }

    /// Advances internal clock by `dt` and produces modulated `AudioSignals`.
    pub fn step(&mut self, dt: f32) -> AudioSignals {
        self.time += dt;
        let beat_freq = (self.bpm / 60.0).max(0.1);
        let phase = (self.time * beat_freq * std::f32::consts::TAU).sin().abs();
        let bass = phase.powi(3);
        let mid = (self.time * 2.1).sin().abs() * 0.6;
        let treble = (self.time * 4.3).cos().abs() * 0.4;
        let volume = (bass * 0.5 + mid * 0.3 + treble * 0.2).clamp(0.0, 1.0);
        AudioSignals::new(bass, mid, treble, volume)
    }
}

impl AudioProvider for SyntheticAudioGenerator {
    fn poll_signals(&mut self) -> AudioSignals {
        self.step(1.0 / 30.0)
    }

    fn is_live(&self) -> bool {
        false
    }

    fn provider_name(&self) -> &'static str {
        "synthetic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_audio_provider() {
        let expected = AudioSignals::new(0.9, 0.4, 0.2, 0.8);
        let mut provider = MockAudioProvider::new(expected);
        assert_eq!(provider.poll_signals(), expected);
        assert_eq!(provider.provider_name(), "mock");
        assert!(!provider.is_live());
    }

    #[test]
    fn test_synthetic_audio_generator_produces_dynamic_signals() {
        let mut gen = SyntheticAudioGenerator::new(120.0);
        assert_eq!(gen.provider_name(), "synthetic");
        assert!(!gen.is_live());
        let s1 = gen.step(0.1);
        let s2 = gen.step(0.2);
        assert!(s1.bass >= 0.0 && s1.bass <= 1.0);
        assert!(s2.bass >= 0.0 && s2.bass <= 1.0);
    }
}
