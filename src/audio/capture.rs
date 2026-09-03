//! Real-time audio stream capture and spectrum analyzer provider.

use super::fft::SpectrumAnalyzer;
use super::provider::{AudioProvider, SyntheticAudioGenerator};
use super::ring_buffer::PcmRingBuffer;
use super::signals::AudioSignals;

use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Real-time audio stream provider consuming from a `PcmRingBuffer` and analyzing via FFT.
/// Features automatic runtime recovery: if the live stream disconnects or errors (`stream_alive == false`),
/// it seamlessly delegates signal generation to an internal `SyntheticAudioGenerator(bpm)` so the visualizer
/// never freezes or flatlines into silence.
pub struct LiveAudioProvider {
    ring_buffer: PcmRingBuffer,
    analyzer: SpectrumAnalyzer,
    sample_buf: Vec<f32>,
    stream_alive: Arc<AtomicBool>,
    fallback_generator: SyntheticAudioGenerator,
    _backend: Option<Box<dyn Any + Send + Sync>>,
}

impl std::fmt::Debug for LiveAudioProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiveAudioProvider")
            .field("ring_buffer", &self.ring_buffer)
            .field("analyzer", &self.analyzer)
            .field("stream_alive", &self.stream_alive.load(Ordering::Relaxed))
            .field("has_backend", &self._backend.is_some())
            .finish()
    }
}

impl LiveAudioProvider {
    /// Creates a new `LiveAudioProvider` with a shared ring buffer and spectrum analyzer.
    pub fn new(ring_buffer: PcmRingBuffer, analyzer: SpectrumAnalyzer) -> Self {
        Self::with_backend(ring_buffer, analyzer, None)
    }

    /// Creates a new `LiveAudioProvider` retaining ownership of an active capture backend.
    pub fn with_backend(
        ring_buffer: PcmRingBuffer,
        analyzer: SpectrumAnalyzer,
        backend: Option<Box<dyn Any + Send + Sync>>,
    ) -> Self {
        let size = analyzer.window_size;
        Self {
            ring_buffer,
            analyzer,
            sample_buf: Vec::with_capacity(size),
            stream_alive: Arc::new(AtomicBool::new(true)),
            fallback_generator: SyntheticAudioGenerator::new(120.0),
            _backend: backend,
        }
    }

    /// Configures the shared stream alive atomic flag.
    pub fn with_stream_alive(mut self, stream_alive: Arc<AtomicBool>) -> Self {
        self.stream_alive = stream_alive;
        self
    }

    /// Sets the shared stream alive atomic flag.
    pub fn set_stream_alive(&mut self, stream_alive: Arc<AtomicBool>) {
        self.stream_alive = stream_alive;
    }

    /// Configures the fallback synthetic generator tempo (BPM).
    pub fn with_fallback_bpm(mut self, bpm: f32) -> Self {
        self.fallback_generator = SyntheticAudioGenerator::new(bpm);
        self
    }

    /// Sets the fallback synthetic generator tempo (BPM).
    pub fn set_fallback_bpm(&mut self, bpm: f32) {
        self.fallback_generator = SyntheticAudioGenerator::new(bpm);
    }

    /// Returns a shared handle to the stream alive flag.
    pub fn stream_alive_handle(&self) -> Arc<AtomicBool> {
        self.stream_alive.clone()
    }

    /// Returns true if the hardware stream is currently running and alive.
    pub fn is_stream_alive(&self) -> bool {
        self.stream_alive.load(Ordering::Relaxed)
    }

    /// Manually signals a stream failure (e.g. for testing recovery).
    pub fn mark_stream_dead(&self) {
        self.stream_alive.store(false, Ordering::SeqCst);
    }

    /// Accessor for the underlying ring buffer to push audio chunks.
    pub fn ring_buffer(&self) -> &PcmRingBuffer {
        &self.ring_buffer
    }

    /// Returns the active sample rate of the spectrum analyzer.
    pub fn sample_rate(&self) -> u32 {
        self.analyzer.sample_rate
    }

    /// Returns the FFT window size of the spectrum analyzer.
    pub fn window_size(&self) -> usize {
        self.analyzer.window_size
    }
}

impl AudioProvider for LiveAudioProvider {
    fn poll_signals(&mut self) -> AudioSignals {
        // Automatic runtime recovery: if stream disconnected or faulted, fallback to synthetic beat generator
        if !self.stream_alive.load(Ordering::Relaxed) {
            return self.fallback_generator.poll_signals();
        }

        self.ring_buffer
            .read_recent(self.analyzer.window_size, &mut self.sample_buf);
        if self.sample_buf.is_empty() {
            AudioSignals::default()
        } else {
            self.analyzer.analyze(&self.sample_buf)
        }
    }

    fn is_live(&self) -> bool {
        self.stream_alive.load(Ordering::Relaxed)
    }

    fn provider_name(&self) -> &'static str {
        "live"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_live_audio_provider_with_ring_buffer() {
        let ring = PcmRingBuffer::new(1024);
        let analyzer = SpectrumAnalyzer::new(44100, 256);
        let mut provider = LiveAudioProvider::new(ring.clone(), analyzer);

        assert!(provider.is_live());
        assert_eq!(provider.provider_name(), "live");
        assert_eq!(provider.sample_rate(), 44100);
        assert_eq!(provider.window_size(), 256);

        // Push silence
        ring.push_slice(&vec![0.0f32; 256]);
        let sig = provider.poll_signals();
        assert_eq!(sig.bass, 0.0);

        // Push loud 100Hz pulse
        let mut pulse = vec![0.0f32; 256];
        for (i, s) in pulse.iter_mut().enumerate() {
            *s = (2.0 * std::f32::consts::PI * 100.0 * i as f32 / 44100.0).sin();
        }
        ring.push_slice(&pulse);
        let sig2 = provider.poll_signals();
        assert!(sig2.bass > 0.0);
    }

    struct DummyBackend {
        stopped: Arc<std::sync::atomic::AtomicBool>,
    }

    impl Drop for DummyBackend {
        fn drop(&mut self) {
            self.stopped
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    #[test]
    fn test_live_audio_provider_retains_backend_ownership_until_dropped() {
        let ring = PcmRingBuffer::new(1024);
        let analyzer = SpectrumAnalyzer::new(44100, 256);
        let stopped_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let backend = DummyBackend {
            stopped: stopped_flag.clone(),
        };

        {
            let provider = LiveAudioProvider::with_backend(ring, analyzer, Some(Box::new(backend)));
            assert!(
                !stopped_flag.load(std::sync::atomic::Ordering::SeqCst),
                "Backend must remain active while provider lives"
            );
            assert!(provider.is_live());
        }

        assert!(
            stopped_flag.load(std::sync::atomic::Ordering::SeqCst),
            "Backend must drop cleanly when provider is dropped"
        );
    }

    #[test]
    fn test_live_audio_provider_runtime_recovery_fallback() {
        let ring = PcmRingBuffer::new(1024);
        let analyzer = SpectrumAnalyzer::new(44100, 256);
        let mut provider = LiveAudioProvider::new(ring.clone(), analyzer);

        assert!(provider.is_live());
        assert!(provider.is_stream_alive());

        // Stream is running with silence
        ring.push_slice(&vec![0.0f32; 256]);
        let silent_sig = provider.poll_signals();
        assert_eq!(silent_sig.bass, 0.0);
        assert_eq!(silent_sig.volume, 0.0);

        // Hardware stream dies (unplugged headphones / DAC error)
        provider.mark_stream_dead();
        assert!(!provider.is_live());
        assert!(!provider.is_stream_alive());

        // Visualizer continues polling; provider automatically synthesizes rhythmic beat
        let fallback_sig1 = provider.poll_signals();
        assert!(
            fallback_sig1.volume > 0.0,
            "Fallback generator must produce active signals to prevent visualizer freezing"
        );

        let fallback_sig2 = provider.poll_signals();
        assert!(fallback_sig2.volume > 0.0);
        // Ensure rhythmic progression over time
        assert_ne!(fallback_sig1, fallback_sig2);

        // Reconnect stream
        provider
            .stream_alive_handle()
            .store(true, std::sync::atomic::Ordering::SeqCst);
        assert!(provider.is_live());

        // Push live pulse again
        let mut pulse = vec![0.0f32; 256];
        for (i, s) in pulse.iter_mut().enumerate() {
            *s = (2.0 * std::f32::consts::PI * 80.0 * i as f32 / 44100.0).sin();
        }
        ring.push_slice(&pulse);
        let recovered_sig = provider.poll_signals();
        assert!(
            recovered_sig.bass > 0.0,
            "Recovered live stream must analyze new PCM samples"
        );
    }
}
