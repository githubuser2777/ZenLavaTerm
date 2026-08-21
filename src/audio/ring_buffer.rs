//! Circular ring buffer for non-blocking asynchronous PCM audio sample ingestion.

use std::sync::{Arc, Mutex};

/// Thread-safe circular ring buffer storing recent PCM audio samples.
#[derive(Debug, Clone)]
pub struct PcmRingBuffer {
    buffer: Arc<Mutex<InnerBuffer>>,
}

#[derive(Debug)]
struct InnerBuffer {
    data: Vec<f32>,
    write_pos: usize,
    capacity: usize,
    total_written: usize,
}

impl PcmRingBuffer {
    /// Creates a new `PcmRingBuffer` with specified sample capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(128);
        Self {
            buffer: Arc::new(Mutex::new(InnerBuffer {
                data: vec![0.0f32; cap],
                write_pos: 0,
                capacity: cap,
                total_written: 0,
            })),
        }
    }

    /// Returns the capacity of the ring buffer.
    pub fn capacity(&self) -> usize {
        self.buffer.lock().map(|b| b.capacity).unwrap_or(0)
    }

    /// Returns the total number of samples pushed into the ring buffer since creation or last clear.
    pub fn total_samples_written(&self) -> usize {
        self.buffer.lock().map(|b| b.total_written).unwrap_or(0)
    }

    /// Clears the ring buffer, filling it with silence (0.0).
    pub fn clear(&self) {
        if let Ok(mut inner) = self.buffer.lock() {
            inner.data.fill(0.0);
            inner.write_pos = 0;
            inner.total_written = 0;
        }
    }

    /// Pushes new mono PCM samples into the circular buffer.
    pub fn push_slice(&self, samples: &[f32]) {
        if samples.is_empty() {
            return;
        }
        if let Ok(mut inner) = self.buffer.lock() {
            let cap = inner.capacity;
            for &sample in samples {
                let pos = inner.write_pos;
                inner.data[pos] = sample;
                inner.write_pos = (pos + 1) % cap;
            }
            inner.total_written = inner.total_written.saturating_add(samples.len());
        }
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
        if let Ok(mut inner) = self.buffer.lock() {
            let cap = inner.capacity;
            for frame_idx in 0..num_frames {
                let start = frame_idx * channels;
                let mut sum = 0.0f32;
                for c in 0..channels {
                    sum += interleaved_samples[start + c];
                }
                let mono_sample = sum * inv_channels;

                let pos = inner.write_pos;
                inner.data[pos] = mono_sample;
                inner.write_pos = (pos + 1) % cap;
            }
            inner.total_written = inner.total_written.saturating_add(num_frames);
        }
    }

    /// Pushes interleaved 16-bit integer PCM samples, normalizing to [-1.0, 1.0] and downmixing to mono.
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

        if let Ok(mut inner) = self.buffer.lock() {
            let cap = inner.capacity;
            for frame_idx in 0..num_frames {
                let start = frame_idx * channels;
                let mut sum = 0.0f32;
                for c in 0..channels {
                    sum += interleaved_samples[start + c] as f32 * inv_scale;
                }
                let mono_sample = (sum * inv_channels).clamp(-1.0, 1.0);

                let pos = inner.write_pos;
                inner.data[pos] = mono_sample;
                inner.write_pos = (pos + 1) % cap;
            }
            inner.total_written = inner.total_written.saturating_add(num_frames);
        }
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

    /// Reads the most recent `count` samples in chronological order.
    pub fn read_recent(&self, count: usize, out: &mut Vec<f32>) {
        out.clear();
        if let Ok(inner) = self.buffer.lock() {
            let cap = inner.capacity;
            let read_len = count.min(cap);
            out.reserve(read_len);

            let start_idx = (inner.write_pos + cap - read_len) % cap;
            for i in 0..read_len {
                let idx = (start_idx + i) % cap;
                out.push(inner.data[idx]);
            }
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
