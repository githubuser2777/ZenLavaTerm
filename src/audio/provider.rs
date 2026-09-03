use super::ring_buffer::PcmRingBuffer;
use super::signals::AudioSignals;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

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

/// Mock stream feeder simulating real hardware audio device frames arriving at real-time intervals.
/// Generates mono or interleaved multi-channel PCM frames (f32, i16, u16) and pushes them into a
/// `PcmRingBuffer`, simulating CPAL hardware callbacks.
pub struct MockAudioStreamFeeder {
    ring_buffer: PcmRingBuffer,
    pub sample_rate: u32,
    pub channels: usize,
    pub frequency: f32,
    stream_alive: Arc<AtomicBool>,
    shutdown_tx: Option<Sender<()>>,
    worker_handle: Option<JoinHandle<()>>,
}

impl std::fmt::Debug for MockAudioStreamFeeder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockAudioStreamFeeder")
            .field("sample_rate", &self.sample_rate)
            .field("channels", &self.channels)
            .field("frequency", &self.frequency)
            .field("stream_alive", &self.stream_alive.load(Ordering::Relaxed))
            .field("is_worker_running", &self.worker_handle.is_some())
            .finish()
    }
}

impl MockAudioStreamFeeder {
    /// Creates a new mock audio stream feeder.
    pub fn new(ring_buffer: PcmRingBuffer, sample_rate: u32, channels: usize) -> Self {
        Self {
            ring_buffer,
            sample_rate: sample_rate.max(8000),
            channels: channels.max(1),
            frequency: 100.0,
            stream_alive: Arc::new(AtomicBool::new(true)),
            shutdown_tx: None,
            worker_handle: None,
        }
    }

    /// Sets the frequency of the generated sine wave.
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Configures the shared stream alive atomic flag.
    pub fn with_stream_alive(mut self, stream_alive: Arc<AtomicBool>) -> Self {
        self.stream_alive = stream_alive;
        self
    }

    /// Returns a shared handle to the stream alive flag.
    pub fn stream_alive_handle(&self) -> Arc<AtomicBool> {
        self.stream_alive.clone()
    }

    /// Feeds a burst of interleaved f32 PCM frames into the ring buffer.
    pub fn feed_frames_f32(&self, num_frames: usize) {
        if num_frames == 0 {
            return;
        }
        let mut buf = Vec::with_capacity(num_frames * self.channels);
        for frame in 0..num_frames {
            let t = frame as f32 / self.sample_rate as f32;
            let s = (t * self.frequency * std::f32::consts::TAU).sin() * 0.7;
            for _ in 0..self.channels {
                buf.push(s);
            }
        }
        self.ring_buffer.push_interleaved_f32(&buf, self.channels);
    }

    /// Feeds a burst of interleaved 16-bit signed integer PCM frames.
    pub fn feed_frames_i16(&self, num_frames: usize) {
        if num_frames == 0 {
            return;
        }
        let mut buf = Vec::with_capacity(num_frames * self.channels);
        for frame in 0..num_frames {
            let t = frame as f32 / self.sample_rate as f32;
            let s = ((t * self.frequency * std::f32::consts::TAU).sin() * 24000.0) as i16;
            for _ in 0..self.channels {
                buf.push(s);
            }
        }
        self.ring_buffer.push_interleaved_i16(&buf, self.channels);
    }

    /// Feeds a burst of interleaved 16-bit unsigned integer PCM frames.
    pub fn feed_frames_u16(&self, num_frames: usize) {
        if num_frames == 0 {
            return;
        }
        let mut buf = Vec::with_capacity(num_frames * self.channels);
        for frame in 0..num_frames {
            let t = frame as f32 / self.sample_rate as f32;
            let s =
                (((t * self.frequency * std::f32::consts::TAU).sin() * 24000.0) + 32768.0) as u16;
            for _ in 0..self.channels {
                buf.push(s);
            }
        }
        self.ring_buffer.push_interleaved_u16(&buf, self.channels);
    }

    /// Starts a background feeder thread generating hardware-like periodic frame chunks.
    pub fn start_worker(&mut self, chunk_frames: usize, interval_ms: u64) {
        self.stop();

        let (shutdown_tx, shutdown_rx) = channel();
        let ring_buffer = self.ring_buffer.clone();
        let sample_rate = self.sample_rate;
        let channels = self.channels;
        let frequency = self.frequency;
        let stream_alive = self.stream_alive.clone();

        let handle = thread::Builder::new()
            .name("mock_audio_feeder".into())
            .spawn(move || {
                let mut frame_index: usize = 0;
                let sleep_duration = Duration::from_millis(interval_ms);
                while shutdown_rx.try_recv().is_err() {
                    if stream_alive.load(Ordering::Relaxed) {
                        let mut buf = Vec::with_capacity(chunk_frames * channels);
                        for _ in 0..chunk_frames {
                            let t = frame_index as f32 / sample_rate as f32;
                            let s = (t * frequency * std::f32::consts::TAU).sin() * 0.7;
                            for _ in 0..channels {
                                buf.push(s);
                            }
                            frame_index = frame_index.wrapping_add(1);
                        }
                        ring_buffer.push_interleaved_f32(&buf, channels);
                    }
                    thread::sleep(sleep_duration);
                }
            })
            .expect("mock feeder thread spawns successfully");

        self.shutdown_tx = Some(shutdown_tx);
        self.worker_handle = Some(handle);
    }

    /// Simulates hardware disconnection or driver error (e.g. unplugging DAC).
    pub fn simulate_disconnect(&self) {
        self.stream_alive.store(false, Ordering::SeqCst);
    }

    /// Simulates hardware reconnection.
    pub fn simulate_reconnect(&self) {
        self.stream_alive.store(true, Ordering::SeqCst);
    }

    /// Returns true if the stream is currently alive.
    pub fn is_stream_alive(&self) -> bool {
        self.stream_alive.load(Ordering::Relaxed)
    }

    /// Stops the background feeder thread.
    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for MockAudioStreamFeeder {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_audio_stream_feeder_f32_burst() {
        let ring = PcmRingBuffer::new(512);
        let feeder = MockAudioStreamFeeder::new(ring.clone(), 44100, 2).with_frequency(220.0);
        assert_eq!(feeder.sample_rate, 44100);
        assert_eq!(feeder.channels, 2);
        assert_eq!(feeder.frequency, 220.0);
        assert!(feeder.is_stream_alive());

        feeder.feed_frames_f32(128);
        assert_eq!(ring.total_samples_written(), 128);

        let mut out = Vec::new();
        ring.read_recent(128, &mut out);
        assert_eq!(out.len(), 128);
        assert!(out.iter().any(|&s| s.abs() > 0.01));
    }

    #[test]
    fn test_mock_audio_stream_feeder_worker_and_disconnect() {
        let ring = PcmRingBuffer::new(1024);
        let mut feeder = MockAudioStreamFeeder::new(ring.clone(), 44100, 1).with_frequency(80.0);

        feeder.start_worker(64, 5);
        thread::sleep(Duration::from_millis(30));

        let written_before = ring.total_samples_written();
        assert!(written_before > 0, "Worker should have pushed frames");

        // Disconnect
        feeder.simulate_disconnect();
        assert!(!feeder.is_stream_alive());
        let count_at_disconnect = ring.total_samples_written();
        thread::sleep(Duration::from_millis(25));
        let count_after_disconnect = ring.total_samples_written();
        assert_eq!(
            count_at_disconnect, count_after_disconnect,
            "Disconnected feeder should not push new frames"
        );

        // Reconnect
        feeder.simulate_reconnect();
        assert!(feeder.is_stream_alive());
        thread::sleep(Duration::from_millis(25));
        assert!(
            ring.total_samples_written() > count_after_disconnect,
            "Reconnected feeder should resume pushing frames"
        );

        feeder.stop();
    }
}
