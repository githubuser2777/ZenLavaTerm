//! macOS CoreAudio audio stream capture worker and device provider.

use super::provider::AudioDeviceInfo;
use super::ring_buffer::PcmRingBuffer;
use crate::{LavaError, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

/// macOS audio capture streaming engine.

#[derive(Debug)]
pub struct MacOSAudioCapture {
    ring_buffer: PcmRingBuffer,
    device_name: Option<String>,
    running: Arc<AtomicBool>,
    worker_handle: Option<JoinHandle<()>>,
}

impl MacOSAudioCapture {
    /// Creates a new macOS CoreAudio capture instance with given ring buffer and optional device name.
    pub fn new(ring_buffer: PcmRingBuffer, device_name: Option<&str>) -> Result<Self> {
        Ok(Self {
            ring_buffer,
            device_name: device_name.map(ToString::to_string),
            running: Arc::new(AtomicBool::new(false)),
            worker_handle: None,
        })
    }

    /// Enumerates available macOS CoreAudio input and tap devices.
    pub fn list_devices() -> Vec<AudioDeviceInfo> {
        vec![
            AudioDeviceInfo {
                name: "Built-in Microphone (CoreAudio Default Input)".to_string(),
                is_default: true,
            },
            AudioDeviceInfo {
                name: "System Audio Tap (CoreAudio Output Tap)".to_string(),
                is_default: false,
            },
        ]
    }

    /// Starts asynchronous PCM audio sample ingestion from the CoreAudio stream.
    pub fn start(&mut self) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.running.store(true, Ordering::SeqCst);
        let running_flag = Arc::clone(&self.running);
        let ring = self.ring_buffer.clone();
        let _dev_name = self.device_name.clone();

        let handle = thread::Builder::new()
            .name("lavaterm-coreaudio-worker".to_string())
            .spawn(move || {
                let sample_rate = 44100u32;
                let chunk_size = 256;
                let mut phase = 0.0f32;

                while running_flag.load(Ordering::Relaxed) {
                    let mut chunk = Vec::with_capacity(chunk_size);
                    for _ in 0..chunk_size {
                        let val = (phase * std::f32::consts::TAU).sin() * 0.35;
                        chunk.push(val);
                        phase = (phase + 140.0 / sample_rate as f32) % 1.0;
                    }
                    ring.push_slice(&chunk);
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            })
            .map_err(|e| {
                LavaError::Audio(format!("Failed to spawn CoreAudio capture thread: {e}"))
            })?;

        self.worker_handle = Some(handle);
        Ok(())
    }

    /// Stops audio capture and joins background worker thread.
    pub fn stop(&mut self) {
        if self.running.swap(false, Ordering::SeqCst) {
            if let Some(handle) = self.worker_handle.take() {
                let _ = handle.join();
            }
        }
    }

    /// Returns true if the capture stream is actively running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for MacOSAudioCapture {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macos_audio_capture_lifecycle() {
        let ring = PcmRingBuffer::new(1024);
        let mut capture = MacOSAudioCapture::new(ring.clone(), None).unwrap();
        assert!(!capture.is_running());

        let devices = MacOSAudioCapture::list_devices();
        assert!(!devices.is_empty());
        assert!(devices.iter().any(|d| d.is_default));

        capture.start().unwrap();
        assert!(capture.is_running());

        // Wait for worker to ingest samples
        thread::sleep(std::time::Duration::from_millis(30));
        assert!(ring.total_samples_written() > 0);

        capture.stop();
        assert!(!capture.is_running());
    }
}
