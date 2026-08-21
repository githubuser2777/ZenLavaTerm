//! Audio reactive signals, FFT spectrum analysis, and audio providers.

pub mod capture;
pub mod fft;
pub mod linux;
pub mod macos;
pub mod provider;
pub mod ring_buffer;
pub mod signals;
pub mod windows;

pub use capture::LiveAudioProvider;
pub use fft::SpectrumAnalyzer;
pub use provider::{AudioDeviceInfo, AudioProvider, MockAudioProvider, SyntheticAudioGenerator};
pub use ring_buffer::{resample_linear, PcmRingBuffer};
pub use signals::AudioSignals;

/// Creates a default audio provider (synthetic beat generator or live capture fallback).
pub fn default_audio_provider() -> Box<dyn AudioProvider> {
    Box::new(SyntheticAudioGenerator::default())
}
