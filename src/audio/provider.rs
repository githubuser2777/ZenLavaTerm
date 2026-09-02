//! Audio provider interfaces, mock fixtures, and synthetic audio generators.

use super::signals::AudioSignals;

/// Trait implemented by audio capture providers.
pub trait AudioProvider: Send {
    fn poll_signals(&mut self) -> AudioSignals;
    fn is_live(&self) -> bool {
        false
    }
    fn provider_name(&self) -> &'static str {
        "unknown"
    }
}

/// Metadata description for an enumerated audio input or loopback device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub direction: &'static str,
}

/// Deterministic mock audio provider for unit testing.
#[derive(Debug, Clone)]
pub struct MockAudioProvider {
    pub signals: AudioSignals,
}

impl MockAudioProvider {
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
    pub time: f32,
    pub bpm: f32,
}

impl Default for SyntheticAudioGenerator {
    fn default() -> Self {
        Self::new(120.0)
    }
}

impl SyntheticAudioGenerator {
    pub fn new(bpm: f32) -> Self {
        Self { time: 0.0, bpm }
    }
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
