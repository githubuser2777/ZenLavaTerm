//! Normalized audio frequency band signals.

/// Normalized audio spectrum bands in range $[0.0, 1.0]$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioSignals {
    /// Sub-bass and bass energy (20 Hz - 250 Hz) in $[0.0, 1.0]$.
    pub bass: f32,
    /// Midrange frequency energy (250 Hz - 4,000 Hz) in $[0.0, 1.0]$.
    pub mid: f32,
    /// Treble and high frequency energy (4,000 Hz - 20,000 Hz) in $[0.0, 1.0]$.
    pub treble: f32,
    /// Overall RMS volume level in $[0.0, 1.0]$.
    pub volume: f32,
}

impl AudioSignals {
    /// Creates a new `AudioSignals` instance with values clamped to $[0.0, 1.0]$.
    pub fn new(bass: f32, mid: f32, treble: f32, volume: f32) -> Self {
        Self {
            bass: bass.clamp(0.0, 1.0),
            mid: mid.clamp(0.0, 1.0),
            treble: treble.clamp(0.0, 1.0),
            volume: volume.clamp(0.0, 1.0),
        }
    }
}

impl Default for AudioSignals {
    fn default() -> Self {
        Self {
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            volume: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_signals_clamping() {
        let sig = AudioSignals::new(1.2, -0.5, 2.0, -0.1);
        assert_eq!(sig.bass, 1.0);
        assert_eq!(sig.mid, 0.0);
        assert_eq!(sig.treble, 1.0);
        assert_eq!(sig.volume, 0.0);
    }
}
