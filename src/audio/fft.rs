//! Fast Fourier Transform (FFT) and frequency band spectrum analyzer.

use super::signals::AudioSignals;
use crate::{LavaError, Result};
use std::f32::consts::PI;

/// Zero-dependency Cooley-Tukey Radix-2 FFT and spectrum analyzer.
#[derive(Debug, Clone)]
pub struct SpectrumAnalyzer {
    /// Audio sample rate in Hz (e.g. 44100 or 48000).
    pub sample_rate: u32,
    /// Analysis window size (must be power of two, e.g. 1024).
    pub window_size: usize,
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            window_size: 1024,
        }
    }
}

impl SpectrumAnalyzer {
    /// Creates a new `SpectrumAnalyzer` with given sample rate and window size.
    pub fn new(sample_rate: u32, window_size: usize) -> Self {
        let size = window_size.next_power_of_two().max(64);
        Self {
            sample_rate,
            window_size: size,
        }
    }

    /// Computes Hann window coefficients.
    pub fn apply_hann_window(samples: &mut [f32]) {
        let n = samples.len();
        if n == 0 {
            return;
        }
        for (i, sample) in samples.iter_mut().enumerate() {
            let mult = 0.5 * (1.0 - (2.0 * PI * i as f32 / (n as f32 - 1.0)).cos());
            *sample *= mult;
        }
    }

    /// Performs in-place Radix-2 Cooley-Tukey FFT on complex input vectors.
    pub fn compute_fft(real: &mut [f32], imag: &mut [f32]) -> Result<()> {
        let n = real.len();
        if n != imag.len() {
            return Err(LavaError::Audio(format!(
                "Real and imaginary buffer lengths must match: {} != {}",
                n,
                imag.len()
            )));
        }
        if n == 0 {
            return Err(LavaError::Audio(
                "FFT buffer length must be greater than zero".to_string(),
            ));
        }
        if !n.is_power_of_two() {
            return Err(LavaError::Audio(format!(
                "FFT size must be a power of two, got {}",
                n
            )));
        }

        // 1. Bit-reversal permutation
        let mut j = 0;
        for i in 0..n - 1 {
            if i < j {
                real.swap(i, j);
                imag.swap(i, j);
            }
            let mut k = n / 2;
            while k <= j {
                j -= k;
                k /= 2;
            }
            j += k;
        }

        // 2. Cooley-Tukey Butterfly passes
        let mut len = 2;
        while len <= n {
            let half = len / 2;
            let angle = -2.0 * PI / len as f32;
            let w_step_re = angle.cos();
            let w_step_im = angle.sin();

            let mut i = 0;
            while i < n {
                let mut w_re = 1.0;
                let mut w_im = 0.0;
                for k in 0..half {
                    let u_re = real[i + k];
                    let u_im = imag[i + k];

                    let v_re = real[i + k + half] * w_re - imag[i + k + half] * w_im;
                    let v_im = real[i + k + half] * w_im + imag[i + k + half] * w_re;

                    real[i + k] = u_re + v_re;
                    imag[i + k] = u_im + v_im;
                    real[i + k + half] = u_re - v_re;
                    imag[i + k + half] = u_im - v_im;

                    let next_w_re = w_re * w_step_re - w_im * w_step_im;
                    let next_w_im = w_re * w_step_im + w_im * w_step_re;
                    w_re = next_w_re;
                    w_im = next_w_im;
                }
                i += len;
            }
            len *= 2;
        }

        Ok(())
    }

    /// Analyzes a slice of PCM samples and returns normalized `AudioSignals`.
    pub fn analyze(&self, pcm: &[f32]) -> AudioSignals {
        if pcm.is_empty() {
            return AudioSignals::default();
        }

        let n = self.window_size;
        // ponytail: dynamic Vec alloc per frame; pre-allocated scratch buffers if audio polling throughput matters
        let mut real = vec![0.0f32; n];
        let mut imag = vec![0.0f32; n];

        let copy_len = pcm.len().min(n);
        real[..copy_len].copy_from_slice(&pcm[..copy_len]);

        // Compute RMS volume
        let mut rms_sum = 0.0f32;
        for &s in &real[..copy_len] {
            rms_sum += s * s;
        }
        let volume = (rms_sum / copy_len as f32).sqrt().min(1.0);

        // Window and FFT
        Self::apply_hann_window(&mut real);
        if let Err(e) = Self::compute_fft(&mut real, &mut imag) {
            eprintln!("Warning: FFT computation failed in spectrum analyzer: {e}");
            return AudioSignals::default();
        }

        // Magnitudes (positive frequencies)
        let num_bins = n / 2;
        let freq_per_bin = self.sample_rate as f32 / n as f32;

        let mut bass_sum = 0.0f32;
        let mut bass_count = 0;
        let mut mid_sum = 0.0f32;
        let mut mid_count = 0;
        let mut treble_sum = 0.0f32;
        let mut treble_count = 0;

        for (bin, (&r, &im)) in real.iter().zip(imag.iter()).enumerate().take(num_bins) {
            let freq = bin as f32 * freq_per_bin;
            let magnitude = (r * r + im * im).sqrt() / (n as f32);

            if (20.0..250.0).contains(&freq) {
                bass_sum += magnitude;
                bass_count += 1;
            } else if (250.0..4000.0).contains(&freq) {
                mid_sum += magnitude;
                mid_count += 1;
            } else if (4000.0..20000.0).contains(&freq) {
                treble_sum += magnitude;
                treble_count += 1;
            }
        }

        let bass_avg = if bass_count > 0 {
            bass_sum / bass_count as f32
        } else {
            0.0
        };
        let mid_avg = if mid_count > 0 {
            mid_sum / mid_count as f32
        } else {
            0.0
        };
        let treble_avg = if treble_count > 0 {
            treble_sum / treble_count as f32
        } else {
            0.0
        };

        // Scale and normalize to [0.0, 1.0]
        let bass = (bass_avg * 15.0).clamp(0.0, 1.0);
        let mid = (mid_avg * 30.0).clamp(0.0, 1.0);
        let treble = (treble_avg * 60.0).clamp(0.0, 1.0);

        AudioSignals::new(bass, mid, treble, volume)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine_wave(freq_hz: f32, sample_rate: u32, count: usize) -> Vec<f32> {
        (0..count)
            .map(|i| (2.0 * PI * freq_hz * i as f32 / sample_rate as f32).sin())
            .collect()
    }

    #[test]
    fn test_fft_sine_100hz_bass_dominant() {
        let analyzer = SpectrumAnalyzer::new(44100, 1024);
        let pcm = generate_sine_wave(100.0, 44100, 1024);
        let signals = analyzer.analyze(&pcm);

        assert!(
            signals.bass > signals.mid,
            "100Hz should have bass > mid (got bass={}, mid={})",
            signals.bass,
            signals.mid
        );
        assert!(
            signals.bass > signals.treble,
            "100Hz should have bass > treble (got bass={}, treble={})",
            signals.bass,
            signals.treble
        );
    }

    #[test]
    fn test_fft_sine_1000hz_mid_dominant() {
        let analyzer = SpectrumAnalyzer::new(44100, 1024);
        let pcm = generate_sine_wave(1000.0, 44100, 1024);
        let signals = analyzer.analyze(&pcm);

        assert!(
            signals.mid > signals.bass,
            "1000Hz should have mid > bass (got mid={}, bass={})",
            signals.mid,
            signals.bass
        );
        assert!(
            signals.mid > signals.treble,
            "1000Hz should have mid > treble (got mid={}, treble={})",
            signals.mid,
            signals.treble
        );
    }

    #[test]
    fn test_fft_sine_8000hz_treble_dominant() {
        let analyzer = SpectrumAnalyzer::new(44100, 1024);
        let pcm = generate_sine_wave(8000.0, 44100, 1024);
        let signals = analyzer.analyze(&pcm);

        assert!(
            signals.treble > signals.bass,
            "8000Hz should have treble > bass (got treble={}, bass={})",
            signals.treble,
            signals.bass
        );
        assert!(
            signals.treble > signals.mid,
            "8000Hz should have treble > mid (got treble={}, mid={})",
            signals.treble,
            signals.mid
        );
    }

    #[test]
    fn test_fft_silence_produces_zero() {
        let analyzer = SpectrumAnalyzer::new(44100, 1024);
        let pcm = vec![0.0f32; 1024];
        let signals = analyzer.analyze(&pcm);

        assert_eq!(signals.bass, 0.0);
        assert_eq!(signals.mid, 0.0);
        assert_eq!(signals.treble, 0.0);
        assert_eq!(signals.volume, 0.0);
    }

    #[test]
    fn test_compute_fft_mismatched_lengths() {
        let mut real = vec![0.0f32; 8];
        let mut imag = vec![0.0f32; 4];
        let res = SpectrumAnalyzer::compute_fft(&mut real, &mut imag);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("lengths must match"));
    }

    #[test]
    fn test_compute_fft_zero_length() {
        let mut real = vec![];
        let mut imag = vec![];
        let res = SpectrumAnalyzer::compute_fft(&mut real, &mut imag);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("greater than zero"));
    }

    #[test]
    fn test_compute_fft_non_power_of_two() {
        let mut real = vec![0.0f32; 10];
        let mut imag = vec![0.0f32; 10];
        let res = SpectrumAnalyzer::compute_fft(&mut real, &mut imag);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("power of two"));
    }

    #[test]
    fn test_compute_fft_valid_power_of_two_and_transform_correctness() {
        // Delta impulse at t=0 -> constant magnitude in frequency domain
        let mut real = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut imag = vec![0.0; 8];
        let res = SpectrumAnalyzer::compute_fft(&mut real, &mut imag);
        assert!(res.is_ok());
        for &r in &real {
            assert!((r - 1.0).abs() < 1e-5);
        }
        for &im in &imag {
            assert!(im.abs() < 1e-5);
        }
    }
}
