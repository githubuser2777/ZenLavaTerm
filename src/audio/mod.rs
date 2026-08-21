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

use crate::config::AudioConfig;
use crate::Result;

/// Enumerates all available audio capture devices across supported platform backends.
pub fn list_audio_devices() -> Vec<AudioDeviceInfo> {
    #[cfg(target_os = "windows")]
    {
        windows::WindowsAudioCapture::list_devices()
    }
    #[cfg(target_os = "macos")]
    {
        macos::MacOSAudioCapture::list_devices()
    }
    #[cfg(target_os = "linux")]
    {
        linux::LinuxAudioCapture::list_devices()
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        vec![AudioDeviceInfo {
            name: "Synthetic Beat Generator".to_string(),
            is_default: true,
        }]
    }
}

/// Creates a default audio provider (synthetic beat generator or live capture fallback).
pub fn default_audio_provider() -> Box<dyn AudioProvider> {
    Box::new(SyntheticAudioGenerator::default())
}

/// Creates a unified audio provider based on configuration, attempting live capture with graceful fallback to synthetic audio.
pub fn create_audio_provider(config: &AudioConfig) -> Box<dyn AudioProvider> {
    if !config.enabled {
        return Box::new(SyntheticAudioGenerator::new(config.bpm));
    }

    match create_live_audio_provider(config.device.as_deref()) {
        Ok(live_provider) => Box::new(live_provider),
        Err(e) => {
            eprintln!("Warning: Live audio capture unavailable ({e}); falling back to synthetic beat generator.");
            Box::new(SyntheticAudioGenerator::new(config.bpm))
        }
    }
}

/// Creates an active `LiveAudioProvider` with the platform-native capture stream running.
pub fn create_live_audio_provider(device_name: Option<&str>) -> Result<LiveAudioProvider> {
    let ring_buffer = PcmRingBuffer::new(4096);
    let analyzer = SpectrumAnalyzer::new(44100, 1024);

    #[cfg(target_os = "windows")]
    let backend: Option<Box<dyn std::any::Any + Send + Sync>> = {
        let mut capture = windows::WindowsAudioCapture::new(ring_buffer.clone(), device_name)?;
        capture.start()?;
        Some(Box::new(capture))
    };

    #[cfg(target_os = "macos")]
    let backend: Option<Box<dyn std::any::Any + Send + Sync>> = {
        let mut capture = macos::MacOSAudioCapture::new(ring_buffer.clone(), device_name)?;
        capture.start()?;
        Some(Box::new(capture))
    };

    #[cfg(target_os = "linux")]
    let backend: Option<Box<dyn std::any::Any + Send + Sync>> = {
        let mut capture = linux::LinuxAudioCapture::new(ring_buffer.clone(), device_name)?;
        capture.start()?;
        Some(Box::new(capture))
    };

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let backend: Option<Box<dyn std::any::Any + Send + Sync>> = None;

    Ok(LiveAudioProvider::with_backend(
        ring_buffer,
        analyzer,
        backend,
    ))
}
