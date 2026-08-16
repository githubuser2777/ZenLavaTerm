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
            })),
        }
    }

    /// Pushes new PCM samples into the circular buffer.
    pub fn push_slice(&self, samples: &[f32]) {
        if let Ok(mut inner) = self.buffer.lock() {
            let cap = inner.capacity;
            for &sample in samples {
                let pos = inner.write_pos;
                inner.data[pos] = sample;
                inner.write_pos = (pos + 1) % cap;
            }
        }
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
    }
}
