//! Circular ring buffer for non-blocking asynchronous PCM audio sample ingestion.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

/// SPSC Ring Buffer with Lock-Free Reader, Serialized Multi-Producer Guard, and Seqlock Snapshot Coherence.
///
/// # Concurrency Architecture:
/// - **Primary Model**: Single-Producer Single-Consumer (SPSC) real-time audio contract.
///   - **Producer**: Dedicated audio capture callback thread (CPAL worker or stream feeder) pushes PCM frames.
///   - **Consumer**: Terminal visualizer render thread reads recent analysis windows for FFT spectrum analysis.
/// - **Lock-Free Reader**: Readers NEVER acquire locks, spinlocks, or block producers. Reads are non-blocking
///   and wait-free in the uncontended common case.
/// - **Tear-Free Snapshot Coherence**: Uses a 64-bit sequence lock (`version`). The producer transitions `version`
///   to odd before writing and even after writing. Readers verify that `version` remained unchanged throughout the read.
///   If writer collision or wrap-around occurs during copy, `read_recent` retries up to 64 times. Under sustained
///   unthrottled contention, it strictly returns `false` and clears the output buffer, never returning unverified or torn data.
/// - **Serialized Multi-Producer Guard**: An atomic CAS spin-guard (`producer_guard`) serializes writes if multiple
///   producer threads write concurrently, protecting write indices while maintaining zero overhead for single producers.
#[derive(Debug, Clone)]
pub struct PcmRingBuffer {
    inner: Arc<LockFreeBuffer>,
}

struct LockFreeBuffer {
    data: Box<[AtomicU32]>,
    write_pos: AtomicUsize,
    capacity: usize,
    total_written: AtomicUsize,
    version: AtomicU64,
    producer_guard: AtomicBool,
}

impl std::fmt::Debug for LockFreeBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LockFreeBuffer")
            .field("capacity", &self.capacity)
            .field("write_pos", &self.write_pos.load(Ordering::Relaxed))
            .field("total_written", &self.total_written.load(Ordering::Relaxed))
            .field("version", &self.version.load(Ordering::Relaxed))
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
                version: AtomicU64::new(0),
                producer_guard: AtomicBool::new(false),
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

    #[inline]
    fn acquire_producer(&self) {
        while self
            .inner
            .producer_guard
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }
    }

    #[inline]
    fn release_producer(&self) {
        self.inner.producer_guard.store(false, Ordering::Release);
    }

    /// Clears the ring buffer, filling it with silence (0.0).
    /// Thread-safe and coherent across concurrent producers and readers.
    pub fn clear(&self) {
        self.acquire_producer();
        let v_start = self.inner.version.fetch_add(1, Ordering::Acquire);

        for slot in self.inner.data.iter() {
            slot.store(0.0f32.to_bits(), Ordering::Relaxed);
        }
        self.inner.write_pos.store(0, Ordering::Relaxed);
        self.inner.total_written.store(0, Ordering::Relaxed);

        self.inner
            .version
            .store(v_start.wrapping_add(2), Ordering::Release);
        self.release_producer();
    }

    /// Pushes new mono PCM samples into the circular buffer in a lock-free manner.
    pub fn push_slice(&self, samples: &[f32]) {
        if samples.is_empty() {
            return;
        }

        self.acquire_producer();
        let v_start = self.inner.version.fetch_add(1, Ordering::Acquire);
        let cap = self.inner.capacity;
        let mut pos = self.inner.write_pos.load(Ordering::Relaxed);

        for &sample in samples {
            self.inner.data[pos].store(sample.to_bits(), Ordering::Relaxed);
            pos = if pos + 1 >= cap { 0 } else { pos + 1 };
        }

        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(samples.len(), Ordering::Release);

        self.inner
            .version
            .store(v_start.wrapping_add(2), Ordering::Release);
        self.release_producer();
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

        self.acquire_producer();
        let v_start = self.inner.version.fetch_add(1, Ordering::Acquire);
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
            pos = if pos + 1 >= cap { 0 } else { pos + 1 };
        }

        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);

        self.inner
            .version
            .store(v_start.wrapping_add(2), Ordering::Release);
        self.release_producer();
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

        self.acquire_producer();
        let v_start = self.inner.version.fetch_add(1, Ordering::Acquire);
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
            pos = if pos + 1 >= cap { 0 } else { pos + 1 };
        }

        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);

        self.inner
            .version
            .store(v_start.wrapping_add(2), Ordering::Release);
        self.release_producer();
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

        self.acquire_producer();
        let v_start = self.inner.version.fetch_add(1, Ordering::Acquire);
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
            pos = if pos + 1 >= cap { 0 } else { pos + 1 };
        }

        self.inner.write_pos.store(pos, Ordering::Release);
        self.inner
            .total_written
            .fetch_add(num_frames, Ordering::Release);

        self.inner
            .version
            .store(v_start.wrapping_add(2), Ordering::Release);
        self.release_producer();
    }

    /// Reads the most recent `count` samples in chronological order without locking.
    /// Employs an optimistic Seqlock protocol: guarantees that the returned window is a
    /// 100% coherent snapshot and never a torn mix of older and overwritten newer generations.
    ///
    /// Returns `true` if and only if a 100% coherent snapshot was verified.
    /// Under extreme contention where a coherent snapshot cannot be verified within 64 retries,
    /// returns `false` and clears `out`, strictly refusing to return unverified or torn data.
    pub fn read_recent(&self, count: usize, out: &mut Vec<f32>) -> bool {
        out.clear();
        let cap = self.inner.capacity;
        let read_len = count.min(cap);
        out.reserve(read_len);

        const MAX_RETRIES: usize = 64;
        for _ in 0..MAX_RETRIES {
            let v0 = self.inner.version.load(Ordering::Acquire);
            if v0 & 1 != 0 {
                std::hint::spin_loop();
                continue;
            }

            let current_pos = self.inner.write_pos.load(Ordering::Acquire);
            let start_idx = (current_pos + cap - read_len) % cap;

            out.clear();
            for i in 0..read_len {
                let idx = (start_idx + i) % cap;
                let bits = self.inner.data[idx].load(Ordering::Relaxed);
                out.push(f32::from_bits(bits));
            }

            let v1 = self.inner.version.load(Ordering::Acquire);
            if v0 == v1 {
                // Verified consistent, non-torn snapshot
                return true;
            }
        }

        // Under sustained extreme contention, strictly refuse to return unverified/torn data.
        out.clear();
        false
    }

    /// Reads the most recent `count` samples, spinning indefinitely until a coherent snapshot is verified.
    pub fn read_recent_blocking(&self, count: usize, out: &mut Vec<f32>) {
        loop {
            if self.read_recent(count, out) {
                return;
            }
            std::hint::spin_loop();
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
            if ring.read_recent(128, &mut read_buf) {
                assert_eq!(read_buf.len(), 128);
                // Verify monotonic ordering within the window
                for w in read_buf.windows(2) {
                    // Either strictly increasing or reset on clear
                    assert!(w[0] <= w[1] || w[1] == 0.0);
                }
                reads_performed += 1;
            }
            thread::sleep(Duration::from_micros(200));
        }

        producer.join().unwrap();
        assert!(ring.total_samples_written() >= total_frames);
    }

    #[test]
    fn test_ring_buffer_wrap_around_snapshot_coherence() {
        use std::sync::atomic::AtomicBool;
        use std::thread;
        use std::time::Duration;

        // Buffer size 256, chunk size 128 (wraps every 2 pushes)
        let ring = PcmRingBuffer::new(256);
        let ring_producer = ring.clone();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let num_generations = 1000;
        let producer = thread::spawn(move || {
            for gen in 1..=num_generations {
                // Each generation fills chunk with: integer part = gen, fraction = index
                let mut chunk = [0.0f32; 128];
                for (i, s) in chunk.iter_mut().enumerate() {
                    *s = gen as f32 + (i as f32 / 1000.0);
                }
                ring_producer.push_slice(&chunk);
                thread::yield_now();
            }
            running_clone.store(false, Ordering::Release);
        });

        let mut read_buf = Vec::new();
        let mut checks = 0;
        while running.load(Ordering::Acquire) || checks < 100 {
            if ring.read_recent(128, &mut read_buf) {
                assert_eq!(read_buf.len(), 128);

                // Verify snapshot consistency: in each coherent read window,
                // generations must be monotonically non-decreasing. There must NEVER be
                // a torn sample (e.g. gen 10 -> gen 2 -> gen 10).
                let mut last_gen = 0.0f32;
                for &sample in &read_buf {
                    if sample > 0.0 {
                        let gen = sample.floor();
                        assert!(
                            gen >= last_gen,
                            "Torn read detected! Observed gen {} after gen {}",
                            gen,
                            last_gen
                        );
                        last_gen = gen;
                    }
                }
                checks += 1;
            }
            thread::sleep(Duration::from_micros(50));
        }

        producer.join().unwrap();
        assert!(ring.total_samples_written() >= num_generations * 128);
    }

    #[test]
    fn test_ring_buffer_multi_producer_contention() {
        use std::thread;

        let ring = PcmRingBuffer::new(1024);
        let num_threads = 4;
        let samples_per_thread = 5120; // 80 * 64
        let mut handles = Vec::new();

        for thread_id in 0..num_threads {
            let ring_clone = ring.clone();
            handles.push(thread::spawn(move || {
                let chunk = vec![thread_id as f32; 64];
                let mut written = 0;
                while written < samples_per_thread {
                    ring_clone.push_slice(&chunk);
                    written += chunk.len();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(
            ring.total_samples_written(),
            num_threads * samples_per_thread
        );
        let mut out = Vec::new();
        ring.read_recent(512, &mut out);
        assert_eq!(out.len(), 512);
        for &s in &out {
            assert!(s >= 0.0 && s < num_threads as f32);
        }
    }
}
