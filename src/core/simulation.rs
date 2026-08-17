//! Simulation orchestrator managing blob states, physics stepping, and scalar field evaluation.

use super::field::ScalarField;
use super::metaball::Blob;
use super::physics::{step_blob, PhysicsParams};

/// Minimal fast XorShift64 PRNG for deterministic noise without external dependencies.
#[derive(Debug, Clone)]
struct SimplePrng {
    state: u64,
}

impl SimplePrng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x5465_726d_4c61_7661
            } else {
                seed
            },
        }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x as u32
    }

    /// Returns a float in $[-1.0, 1.0]$.
    fn next_f32_signed(&mut self) -> f32 {
        let val = (self.next_u32() & 0xFFFF) as f32 / 65535.0;
        val * 2.0 - 1.0
    }
}

/// The central LavaTerm simulation container.
#[derive(Debug, Clone)]
pub struct Simulation {
    /// Active metaballs.
    pub blobs: Vec<Blob>,
    /// Physics parameters.
    pub params: PhysicsParams,
    /// Scalar field evaluator.
    pub field: ScalarField,
    /// Total elapsed simulation time in seconds.
    pub elapsed_time: f32,
    /// Base radius scale for compact / adaptive viewports.
    pub radius_scale: f32,
    /// Internal deterministic PRNG.
    prng: SimplePrng,
}

impl Simulation {
    /// Maximum allowable delta time to avoid simulation instability.
    pub const MAX_DT: f32 = 0.10;

    /// Creates a new simulation with `blob_count` blobs arranged with initial thermal distribution.
    pub fn new(params: PhysicsParams, blob_count: usize, seed: u64) -> Self {
        let mut prng = SimplePrng::new(seed);
        let mut blobs = Vec::with_capacity(blob_count);

        let count = blob_count.max(1);
        for i in 0..count {
            let frac = (i as f32 + 0.5) / count as f32;
            let x = 0.15 + 0.70 * frac;
            // Half start near bottom, half start near top with alternating temperatures
            let is_bottom = i % 2 == 0;
            let y = if is_bottom {
                0.10 + 0.10 * prng.next_f32_signed()
            } else {
                0.85 + 0.10 * prng.next_f32_signed()
            };
            let temp = if is_bottom { 0.90 } else { 0.10 };
            let radius = 0.08 + 0.04 * (prng.next_f32_signed().abs());

            let mut blob = Blob::new(x, y, radius, temp);
            blob.vx = prng.next_f32_signed() * 0.05;
            blob.vy = prng.next_f32_signed() * 0.05;
            blobs.push(blob);
        }

        Self {
            blobs,
            params,
            field: ScalarField,
            elapsed_time: 0.0,
            radius_scale: 1.0,
            prng,
        }
    }

    /// Scales all blob radii by `scale` and sets `self.radius_scale`.
    pub fn apply_radius_scale(&mut self, scale: f32) {
        self.radius_scale = scale;
        if (scale - 1.0).abs() > f32::EPSILON {
            for blob in &mut self.blobs {
                blob.radius = (blob.radius * scale).max(0.01);
            }
        }
    }

    /// Advances the simulation by $\Delta t$ seconds.
    pub fn step(&mut self, dt: f32) {
        let dt_effective = dt.clamp(0.0, Self::MAX_DT);
        self.elapsed_time += dt_effective;

        for blob in &mut self.blobs {
            let noise_x = self.prng.next_f32_signed();
            let noise_y = self.prng.next_f32_signed();
            step_blob(blob, &self.params, dt_effective, (noise_x, noise_y));
        }
    }

    /// Evaluates the scalar field at coordinate `(px, py)` in $[0.0, 1.0] \times [0.0, 1.0]$.
    #[inline]
    pub fn evaluate_field(&self, px: f32, py: f32) -> f32 {
        self.field.evaluate_field(&self.blobs, px, py)
    }

    /// Evaluates field intensity and temperature at `(px, py)`.
    #[inline]
    pub fn evaluate_at(&self, px: f32, py: f32) -> (f32, f32) {
        self.field.evaluate_with_temperature(&self.blobs, px, py)
    }

    /// Dynamically modulates physics constants and blob parameters from normalized reactive signals.
    pub fn apply_signals(&mut self, signals: &crate::reactive::SystemSignals) {
        // CPU load modulates Brownian thermal noise and turbulence (base * (1.0 + cpu * 2.5))
        self.params.noise = 0.15 * (1.0 + signals.cpu_load * 2.5);

        // Battery level modulates upward buoyancy (low battery -> slower sluggish lava, high battery -> lively convection)
        self.params.buoyancy = 0.50 + signals.battery_level * 0.60;

        // RAM memory usage modulates blob size: higher memory -> larger, expanding blobs
        let radius_multiplier = 0.85 + signals.memory_usage * 0.40;
        for (i, blob) in self.blobs.iter_mut().enumerate() {
            let base_r = 0.08 + 0.04 * ((i % 3) as f32 / 3.0);
            blob.radius = (base_r * self.radius_scale * radius_multiplier).max(0.01);
        }
    }

    /// Advances the simulation by $\Delta t$ seconds while applying reactive system signals.
    pub fn step_reactive(&mut self, dt: f32, signals: &crate::reactive::SystemSignals) {
        self.apply_signals(signals);
        self.step(dt);
    }

    /// Dynamically modulates physics constants and blob kinetics from normalized audio spectrum signals.
    pub fn apply_audio_signals(&mut self, audio: &crate::audio::AudioSignals) {
        // Bass kick gives strong convective thrust and upward buoyancy surge
        self.params.buoyancy = 0.80 + audio.bass * 1.50;

        // Midrange frequencies modulate fluid turbulence and Brownian noise
        self.params.noise = 0.15 * (1.0 + audio.mid * 2.5);

        // Treble adds subtle kinetic energy jitter
        if audio.treble > 0.3 {
            for blob in &mut self.blobs {
                blob.vx += (self.prng.next_f32_signed()) * audio.treble * 0.02;
            }
        }
    }

    /// Advances the simulation by $\Delta t$ seconds while applying reactive audio signals.
    pub fn step_audio(&mut self, dt: f32, audio: &crate::audio::AudioSignals) {
        self.apply_audio_signals(audio);
        self.step(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::AudioSignals;
    use crate::reactive::SystemSignals;

    #[test]
    fn test_simulation_step_deterministic() {
        let mut sim1 = Simulation::new(PhysicsParams::default(), 6, 42);
        let mut sim2 = Simulation::new(PhysicsParams::default(), 6, 42);

        for _ in 0..10 {
            sim1.step(0.033);
            sim2.step(0.033);
        }

        assert_eq!(sim1.blobs.len(), sim2.blobs.len());
        for (b1, b2) in sim1.blobs.iter().zip(sim2.blobs.iter()) {
            assert!((b1.x - b2.x).abs() < 1e-6);
            assert!((b1.y - b2.y).abs() < 1e-6);
        }
    }

    #[test]
    fn test_simulation_bounds_dt() {
        let mut sim = Simulation::new(PhysicsParams::default(), 4, 123);
        sim.step(10.0); // Extreme pause
        assert!(sim.elapsed_time <= Simulation::MAX_DT + 1e-5);
    }

    #[test]
    fn test_simulation_reactive_signal_modulation() {
        let mut sim = Simulation::new(PhysicsParams::default(), 6, 42);
        let low_signals = SystemSignals::new(0.0, 0.0, 0.0, 0.0);
        let high_signals = SystemSignals::new(1.0, 1.0, 1.0, 1.0);

        sim.apply_signals(&low_signals);
        let low_noise = sim.params.noise;
        let low_buoyancy = sim.params.buoyancy;
        let low_radius = sim.blobs[0].radius;

        sim.apply_signals(&high_signals);
        let high_noise = sim.params.noise;
        let high_buoyancy = sim.params.buoyancy;
        let high_radius = sim.blobs[0].radius;

        assert!(
            high_noise > low_noise,
            "CPU load must increase turbulence noise"
        );
        assert!(
            high_buoyancy > low_buoyancy,
            "Battery charge must increase buoyancy"
        );
        assert!(
            high_radius > low_radius,
            "Memory usage must increase blob size"
        );
    }

    #[test]
    fn test_simulation_audio_signal_modulation() {
        let mut sim = Simulation::new(PhysicsParams::default(), 6, 42);
        let quiet = AudioSignals::new(0.0, 0.0, 0.0, 0.0);
        let drop = AudioSignals::new(1.0, 0.8, 0.6, 0.9);

        sim.apply_audio_signals(&quiet);
        let quiet_buoyancy = sim.params.buoyancy;
        let quiet_noise = sim.params.noise;

        sim.apply_audio_signals(&drop);
        let drop_buoyancy = sim.params.buoyancy;
        let drop_noise = sim.params.noise;

        assert!(
            drop_buoyancy > quiet_buoyancy,
            "Bass kick must increase buoyancy"
        );
        assert!(
            drop_noise > quiet_noise,
            "Midrange audio must increase turbulence noise"
        );
    }
}
