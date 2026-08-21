//! Real-time audio stream capture and spectrum analyzer provider.

use super::fft::SpectrumAnalyzer;
use super::provider::AudioProvider;
use super::ring_buffer::PcmRingBuffer;
use super::signals::AudioSignals;

/// Real-time audio stream provider consuming from a `PcmRingBuffer` and analyzing via FFT.
#[derive(Debug, Clone)]
pub struct LiveAudioProvider {
    ring_buffer: PcmRingBuffer,
    analyzer: SpectrumAnalyzer,
    sample_buf: Vec<f32>,
}

impl LiveAudioProvider {
    /// Creates a new `LiveAudioProvider` with a shared ring buffer and spectrum analyzer.
    pub fn new(ring_buffer: PcmRingBuffer, analyzer: SpectrumAnalyzer) -> Self {
        let size = analyzer.window_size;
        Self {
            ring_buffer,
            analyzer,
            sample_buf: Vec::with_capacity(size),
        }
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
        self.ring_buffer
            .read_recent(self.analyzer.window_size, &mut self.sample_buf);
        if self.sample_buf.is_empty() {
            AudioSignals::default()
        } else {
            self.analyzer.analyze(&self.sample_buf)
        }
    }

    fn is_live(&self) -> bool {
        true
    }

    fn provider_name(&self) -> &'static str {
        "live"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
