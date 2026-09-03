//! Circular ring buffer for non-blocking asynchronous PCM audio sample ingestion.

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;

/// Lock-free circular ring buffer storing recent PCM audio samples.
/// Designed for real-time audio threads: sample ingestion and reads are non-blocking,
/// atomic, and lock-free (no mutexes or syscalls in audio path).
#[derive(Debug, Clone)]
pub struct PcmRingBuffer {
    inner: Arc<LockFreeBuffer>,
}

struct LockFreeBuffer {
    data: Box<[AtomicU32]>,
    write_pos: AtomicUsize,
    capacity: usize,
    total_written: AtomicUsize,
}

impl std::fmt::Debug for LockFreeBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LockFreeBuffer")
            .field("capacity", &self.capacity)
            .field("write_pos", &self.write_pos.load(Ordering::Relaxed))
            .field("total_written", &self.total_written.load(Ordering::Relaxed))
            .finish()
    }
}

impl PcmRingBuffer {
    /// Creates a new `PcmRingBuffer` with specified sample capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(128);
        let data: Vec<AtomicU32> = (0..cap).map(|_| AtomicU32::new(0.0f32.to_bits())).collect();
        Self {
            inner: Arc::new(LockFreeBuffer {
                data: data.into_boxed_slice(),
                write_pos: AtomicUsize::new(0),
                capacity: cap,
                total_written: AtomicUsize::new(0),
            }),
        }
    }

    /// Returns the capacity of the ring buffer.
    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }

    /// Returns the total number of samples pushed into the ring buffer since creation or last clear.
    pub fn total_samples_written(&self) -> usize {
        self.inner.total_written.load(Ordering::Acquire)
    }

    /// Clears the ring buffer, filling it with silence (0.0).
    pub fn clear(&self) {
        for slot in self.inner.data.iter() {
            slot.store(0.0f32.to_bits(), Ordering::Release);
        }
        self.inner.write_pos.store(0, Ordering::Release);
        self.inner.total_written.store(0, Ordering::Release);
    }

    /// Pushes new mono PCM samples into the circular buffer in a lock-free manner.
    pub fn push_slice(&self, samples: &[f32]) {
        if samples.is_empty() {
            return;
        }
        let cap = self.inner.capacity;
        let mut pos = self.inner.write_pos.load(Ordering::Relaxed);
        for &sample in samples {
            self.inner.data[pos].store(sample.to_bits(), Ordering::Relaxed);
            pos += 1;
            if pos >= cap {
                pos = 0;
            }
        }
        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(samples.len(), Ordering::Release);
    }

    /// Pushes interleaved multi-channel f32 samples by downmixing to mono.
    pub fn push_interleaved_f32(&self, interleaved_samples: &[f32], channels: usize) {
        if interleaved_samples.is_empty() || channels == 0 {
            return;
        }

        if channels == 1 {
            self.push_slice(interleaved_samples);
            return;
        }

        let num_frames = interleaved_samples.len() / channels;
        if num_frames == 0 {
            return;
        }

        let inv_channels = 1.0 / (channels as f32);
        let cap = self.inner.capacity;
        let mut pos = self.inner.write_pos.load(Ordering::Relaxed);
        for frame_idx in 0..num_frames {
            let start = frame_idx * channels;
            let mut sum = 0.0f32;
            for c in 0..channels {
                sum += interleaved_samples[start + c];
            }
            let mono_sample = sum * inv_channels;

            self.inner.data[pos].store(mono_sample.to_bits(), Ordering::Relaxed);
            pos += 1;
            if pos >= cap {
                pos = 0;
            }
        }
        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);
    }

    /// Pushes interleaved 16-bit unsigned integer PCM samples, normalizing to [-1.0, 1.0] and downmixing to mono.
    pub fn push_interleaved_u16(&self, interleaved_samples: &[u16], channels: usize) {
        if interleaved_samples.is_empty() || channels == 0 {
            return;
        }

        let num_frames = interleaved_samples.len() / channels;

        if num_frames == 0 {
            return;
        }

        let inv_scale = 1.0 / 32768.0f32;
        let inv_channels = 1.0 / (channels as f32);
        let cap = self.inner.capacity;
        let mut pos = self.inner.write_pos.load(Ordering::Relaxed);

        for frame_idx in 0..num_frames {
            let start = frame_idx * channels;
            let mut sum = 0.0f32;
            for c in 0..channels {
                let s = (interleaved_samples[start + c] as f32 - 32768.0) * inv_scale;
                sum += s;
            }
            let mono_sample = (sum * inv_channels).clamp(-1.0, 1.0);

            self.inner.data[pos].store(mono_sample.to_bits(), Ordering::Relaxed);
            pos += 1;
            if pos >= cap {
                pos = 0;
            }
        }
        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);
    }

    /// Pushes interleaved 16-bit signed integer PCM samples, normalizing to [-1.0, 1.0] and downmixing to mono.
    pub fn push_interleaved_i16(&self, interleaved_samples: &[i16], channels: usize) {
        if interleaved_samples.is_empty() || channels == 0 {
            return;
        }

        let num_frames = interleaved_samples.len() / channels;
        if num_frames == 0 {
            return;
        }

        let inv_scale = 1.0 / 32768.0f32;
        let inv_channels = 1.0 / (channels as f32);
        let cap = self.inner.capacity;
        let mut pos = self.inner.write_pos.load(Ordering::Relaxed);

        for frame_idx in 0..num_frames {
            let start = frame_idx * channels;
            let mut sum = 0.0f32;
            for c in 0..channels {
                sum += interleaved_samples[start + c] as f32 * inv_scale;
            }
            let mono_sample = (sum * inv_channels).clamp(-1.0, 1.0);

            self.inner.data[pos].store(mono_sample.to_bits(), Ordering::Relaxed);
            pos += 1;
            if pos >= cap {
                pos = 0;
            }
        }
        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);
    }

    /// Pushes mono samples with linear resampling from `src_rate` to `dst_rate`.
    pub fn push_resampled(&self, samples: &[f32], src_rate: u32, dst_rate: u32) {
        if samples.is_empty() {
            return;
        }
        if src_rate == dst_rate || src_rate == 0 || dst_rate == 0 {
            self.push_slice(samples);
            return;
        }

        let ratio = src_rate as f64 / dst_rate as f64;
        let output_len = ((samples.len() as f64) / ratio).round() as usize;
        if output_len == 0 {
            return;
        }

        let mut resampled = Vec::with_capacity(output_len);
        for i in 0..output_len {
            let src_idx = i as f64 * ratio;
            let idx0 = src_idx.floor() as usize;
            let frac = (src_idx - idx0 as f64) as f32;

            if idx0 + 1 < samples.len() {
                let s0 = samples[idx0];
                let s1 = samples[idx0 + 1];
                resampled.push(s0 + frac * (s1 - s0));
            } else if idx0 < samples.len() {
                resampled.push(samples[idx0]);
            }
        }

        self.push_slice(&resampled);
    }

    /// Reads the most recent `count` samples in chronological order without locking.
    pub fn read_recent(&self, count: usize, out: &mut Vec<f32>) {
        out.clear();
        let cap = self.inner.capacity;
        let read_len = count.min(cap);
        out.reserve(read_len);

        let current_pos = self.inner.write_pos.load(Ordering::Acquire);
        let start_idx = (current_pos + cap - read_len) % cap;
        for i in 0..read_len {
            let idx = (start_idx + i) % cap;
            let bits = self.inner.data[idx].load(Ordering::Relaxed);
            out.push(f32::from_bits(bits));
        }
    }
}

/// Helper function to linearly resample a PCM slice.
pub fn resample_linear(input: &[f32], src_rate: u32, dst_rate: u32, out: &mut Vec<f32>) {
    out.clear();
    if input.is_empty() || src_rate == 0 || dst_rate == 0 {
        return;
    }
    if src_rate == dst_rate {
        out.extend_from_slice(input);
        return;
    }

    let ratio = src_rate as f64 / dst_rate as f64;
    let output_len = ((input.len() as f64) / ratio).round() as usize;
    if output_len == 0 {
        return;
    }

    out.reserve(output_len);
    for i in 0..output_len {
        let src_idx = i as f64 * ratio;
        let idx0 = src_idx.floor() as usize;
        let frac = (src_idx - idx0 as f64) as f32;

        if idx0 + 1 < input.len() {
            let s0 = input[idx0];
            let s1 = input[idx0 + 1];
            out.push(s0 + frac * (s1 - s0));
        } else if idx0 < input.len() {
            out.push(input[idx0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_push_and_read() {
        let ring = PcmRingBuffer::new(8);
        ring.push_slice(&[1.0, 2.0, 3.0, 4.0]);

        let mut read_out = Vec::new();
        ring.read_recent(4, &mut read_out);
        assert_eq!(read_out, vec![1.0, 2.0, 3.0, 4.0]);

        // Push more to wrap around
        ring.push_slice(&[5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        ring.read_recent(4, &mut read_out);
        assert_eq!(read_out, vec![7.0, 8.0, 9.0, 10.0]);
        assert_eq!(ring.total_samples_written(), 10);
    }

    #[test]
    fn test_ring_buffer_push_interleaved_f32_stereo() {
        let ring = PcmRingBuffer::new(8);
        // Stereo: L=1.0, R=0.5 -> avg 0.75; L=-0.5, R=0.5 -> avg 0.0
        let stereo = vec![1.0f32, 0.5, -0.5, 0.5];
        ring.push_interleaved_f32(&stereo, 2);

        let mut read_out = Vec::new();
        ring.read_recent(2, &mut read_out);
        assert_eq!(read_out.len(), 2);
        assert!((read_out[0] - 0.75).abs() < 1e-6);
        assert!((read_out[1] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_ring_buffer_push_interleaved_i16() {
        let ring = PcmRingBuffer::new(8);
        // Stereo i16: 32767 -> ~1.0, 0 -> 0.0, -32768 -> -1.0
        let stereo = vec![32767i16, 32767i16, -32768i16, 32767i16];
        ring.push_interleaved_i16(&stereo, 2);

        let mut read_out = Vec::new();
        ring.read_recent(2, &mut read_out);
        assert_eq!(read_out.len(), 2);
        assert!((read_out[0] - 1.0).abs() < 1e-3);
        assert!((read_out[1] - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_ring_buffer_resampling_linear() {
        let mut out = Vec::new();
        let input = vec![0.0f32, 1.0, 2.0, 3.0, 4.0];
        // Resample 2x up
        resample_linear(&input, 100, 200, &mut out);
        assert_eq!(out.len(), 10);
        assert_eq!(out[0], 0.0);
        assert_eq!(out[2], 1.0);
    }

    #[test]
    fn test_ring_buffer_clear() {
        let ring = PcmRingBuffer::new(8);
        ring.push_slice(&[1.0, 2.0, 3.0, 4.0]);
        ring.clear();
        assert_eq!(ring.total_samples_written(), 0);

        let mut read_out = Vec::new();
        ring.read_recent(4, &mut read_out);
        assert_eq!(read_out, vec![0.0, 0.0, 0.0, 0.0]);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_ring_buffer_push_interleaved_u16() {
        let ring = PcmRingBuffer::new(8);
        // Stereo u16: 65535 -> ~1.0, 32768 -> 0.0, 0 -> -1.0
        let stereo = vec![65535u16, 65535u16, 0u16, 65535u16];
        ring.push_interleaved_u16(&stereo, 2);

        let mut read_out = Vec::new();
        ring.read_recent(2, &mut read_out);
        assert_eq!(read_out.len(), 2);
        assert!((read_out[0] - 1.0).abs() < 1e-3);
        assert!((read_out[1] - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_ring_buffer_lock_free_concurrent_producer_consumer() {
        use std::sync::atomic::AtomicBool;
        use std::thread;
        use std::time::Duration;

        let ring = PcmRingBuffer::new(512);
        let ring_producer = ring.clone();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let total_frames = 20_000;
        let producer = thread::spawn(move || {
            let mut val = 0.0f32;
            let mut chunk = [0.0f32; 64];
            let mut written = 0;
            while written < total_frames {
                for s in chunk.iter_mut() {
                    *s = val;
                    val += 1.0;
                }
                ring_producer.push_slice(&chunk);
                written += chunk.len();
                thread::yield_now();
            }
            running_clone.store(false, Ordering::Release);
        });

        let mut read_buf = Vec::new();
        let mut reads_performed = 0;
        while running.load(Ordering::Acquire) || reads_performed < 50 {
            ring.read_recent(128, &mut read_buf);
            assert_eq!(read_buf.len(), 128);
            // Verify monotonic ordering within the window
            for w in read_buf.windows(2) {
                // Either strictly increasing or reset on clear
                assert!(w[0] <= w[1] || w[1] == 0.0);
            }
            reads_performed += 1;
            thread::sleep(Duration::from_micros(200));
        }

        producer.join().unwrap();
        assert!(ring.total_samples_written() >= total_frames);
    }
}
