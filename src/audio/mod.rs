//! Audio reactive signals, FFT spectrum analysis, and audio providers.

pub mod capture;
pub mod fft;
pub mod native;
pub mod provider;
pub mod ring_buffer;
pub mod signals;

pub use capture::LiveAudioProvider;
pub use fft::SpectrumAnalyzer;
pub use provider::{AudioDeviceInfo, AudioProvider, MockAudioProvider, SyntheticAudioGenerator};
pub use ring_buffer::{resample_linear, PcmRingBuffer};
pub use signals::AudioSignals;

use crate::config::AudioConfig;
use crate::Result;

/// Enumerates all available audio capture devices across supported platform backends.
pub fn list_audio_devices() -> Vec<AudioDeviceInfo> {
    native::NativeAudioCapture::list_devices()
}

/// Creates a default audio provider (synthetic beat generator or live capture fallback).
pub fn default_audio_provider() -> Box<dyn AudioProvider> {
    Box::new(SyntheticAudioGenerator::default())
}

/// Creates a unified audio provider based on configuration.
/// If audio is explicitly enabled, it strictly attempts to open a live capture stream.
/// Failure to open the native stream will result in an error, NOT a silent fallback to synthetic audio.
pub fn create_audio_provider(config: &AudioConfig) -> Result<Box<dyn AudioProvider>> {
    if !config.enabled {
        return Ok(Box::new(SyntheticAudioGenerator::new(config.bpm)));
    }

    let live_provider = create_live_audio_provider(config.device.as_deref(), config.loopback)?;
    Ok(Box::new(live_provider))
}

/// Creates an active `LiveAudioProvider` with the platform-native capture stream running.
pub fn create_live_audio_provider(device_name: Option<&str>, loopback: bool) -> Result<LiveAudioProvider> {
    let ring_buffer = PcmRingBuffer::new(4096);
    let capture = native::NativeAudioCapture::new(ring_buffer.clone(), device_name, loopback)?;
    let analyzer = SpectrumAnalyzer::new(capture.actual_sample_rate, 1024);

    let backend: Option<Box<dyn std::any::Any + Send + Sync>> = Some(Box::new(capture));

    Ok(LiveAudioProvider::with_backend(
        ring_buffer,
        analyzer,
        backend,
    ))
}
