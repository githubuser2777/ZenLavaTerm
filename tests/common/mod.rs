//! Test helpers and mock fixtures for LavaTerm integration tests.

use lavaterm::audio::PcmRingBuffer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

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
