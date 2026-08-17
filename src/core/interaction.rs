//! Interactive physics models for mouse clicks, dragging, scrolling, and keyboard ripples.

use super::metaball::Blob;
use super::physics::PhysicsParams;

/// High-level interaction domain representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interaction {
    /// Radial shockwave impulse from mouse click centered at `(x, y)` in $[0.0, 1.0]$.
    Shockwave { x: f32, y: f32, force: f32 },
    /// Fluid stirring from mouse dragging, transferring velocity `(vx, vy)` within `radius` around `(x, y)`.
    Stir {
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        radius: f32,
    },
    /// Global acoustic wave ripple from keyboard press.
    Ripple { intensity: f32 },
    /// Vertical pressure wave from mouse scroll.
    Pressure { delta: f32 },
    /// Localized thermal injection (heating or cooling).
    ThermalPulse {
        x: f32,
        y: f32,
        temperature_delta: f32,
        radius: f32,
    },
}

/// Applies a radial shockwave pushing blobs away from `(center_x, center_y)`.
pub fn apply_shockwave(blobs: &mut [Blob], center_x: f32, center_y: f32, force: f32) {
    let cx = center_x.clamp(0.0, 1.0);
    let cy = center_y.clamp(0.0, 1.0);
    let force_clamped = force.clamp(0.0, 10.0);

    for blob in blobs.iter_mut() {
        let dx = blob.x - cx;
        let dy = blob.y - cy;
        let dist = (dx * dx + dy * dy).sqrt() + 0.001;

        let dir_x = dx / dist;
        let dir_y = dy / dist;

        // Smooth inverse-distance falloff with soft core
        let impulse = (force_clamped * 0.08) / (dist * dist + 0.04);
        let impulse_clamped = impulse.min(1.0);

        blob.vx = (blob.vx + dir_x * impulse_clamped).clamp(-2.0, 2.0);
        blob.vy = (blob.vy + dir_y * impulse_clamped).clamp(-2.0, 2.0);

        // Shockwave causes thermal agitation
        let thermal_gain = (force_clamped * 0.05) / (dist + 0.1);
        blob.temperature = (blob.temperature + thermal_gain).clamp(0.0, 1.0);
    }
}

/// Applies fluid stirring from drag motion, transferring momentum to blobs within `radius`.
pub fn apply_stir(blobs: &mut [Blob], x: f32, y: f32, vx: f32, vy: f32, radius: f32) {
    let cx = x.clamp(0.0, 1.0);
    let cy = y.clamp(0.0, 1.0);
    let r = radius.max(0.01);
    let r_sq = r * r;

    let vx_clamped = vx.clamp(-2.0, 2.0);
    let vy_clamped = vy.clamp(-2.0, 2.0);

    for blob in blobs.iter_mut() {
        let dx = blob.x - cx;
        let dy = blob.y - cy;
        let d_sq = dx * dx + dy * dy;

        if d_sq < r_sq {
            let dist = d_sq.sqrt();
            let weight = (1.0 - (dist / r)).clamp(0.0, 1.0);

            blob.vx = (blob.vx + vx_clamped * weight * 0.6).clamp(-2.0, 2.0);
            blob.vy = (blob.vy + vy_clamped * weight * 0.6).clamp(-2.0, 2.0);
        }
    }
}

/// Applies global acoustic ripple to all blobs.
pub fn apply_ripple<F>(blobs: &mut [Blob], intensity: f32, mut prng_fn: F)
where
    F: FnMut() -> f32,
{
    let i_clamped = intensity.clamp(0.0, 5.0);
    for blob in blobs.iter_mut() {
        let rx = prng_fn();
        let ry = prng_fn();
        blob.vx = (blob.vx + rx * i_clamped * 0.05).clamp(-2.0, 2.0);
        blob.vy = (blob.vy + ry * i_clamped * 0.05).clamp(-2.0, 2.0);
    }
}

/// Applies pressure modulation to physics parameters and convective state.
pub fn apply_pressure(params: &mut PhysicsParams, delta: f32) {
    params.buoyancy = (params.buoyancy + delta * 0.15).clamp(0.1, 3.0);
}

/// Applies localized heating or cooling to blobs in `radius`.
pub fn apply_thermal_pulse(
    blobs: &mut [Blob],
    x: f32,
    y: f32,
    temperature_delta: f32,
    radius: f32,
) {
    let cx = x.clamp(0.0, 1.0);
    let cy = y.clamp(0.0, 1.0);
    let r = radius.max(0.01);
    let r_sq = r * r;

    for blob in blobs.iter_mut() {
        let dx = blob.x - cx;
        let dy = blob.y - cy;
        let d_sq = dx * dx + dy * dy;

        if d_sq < r_sq {
            let dist = d_sq.sqrt();
            let weight = (1.0 - (dist / r)).clamp(0.0, 1.0);
            blob.temperature = (blob.temperature + temperature_delta * weight).clamp(0.0, 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shockwave_repels_blobs_radially() {
        let mut blobs = vec![
            Blob::new(0.5, 0.6, 0.1, 0.5), // Above center
            Blob::new(0.5, 0.4, 0.1, 0.5), // Below center
            Blob::new(0.6, 0.5, 0.1, 0.5), // Right of center
            Blob::new(0.4, 0.5, 0.1, 0.5), // Left of center
        ];

        apply_shockwave(&mut blobs, 0.5, 0.5, 1.0);

        assert!(blobs[0].vy > 0.0, "Blob above center must be pushed upward");
        assert!(
            blobs[1].vy < 0.0,
            "Blob below center must be pushed downward"
        );
        assert!(
            blobs[2].vx > 0.0,
            "Blob right of center must be pushed rightward"
        );
        assert!(
            blobs[3].vx < 0.0,
            "Blob left of center must be pushed leftward"
        );
    }

    #[test]
    fn test_shockwave_falloff_with_distance() {
        let mut blobs = vec![
            Blob::new(0.5, 0.55, 0.1, 0.5), // Near (dist 0.05)
            Blob::new(0.5, 0.85, 0.1, 0.5), // Far (dist 0.35)
        ];

        apply_shockwave(&mut blobs, 0.5, 0.5, 1.0);

        assert!(
            blobs[0].vy > blobs[1].vy,
            "Closer blob must receive stronger impulse than farther blob"
        );
    }

    #[test]
    fn test_stir_momentum_transfer() {
        let mut blobs = vec![
            Blob::new(0.5, 0.5, 0.1, 0.5), // Inside radius
            Blob::new(0.9, 0.9, 0.1, 0.5), // Outside radius
        ];

        apply_stir(&mut blobs, 0.5, 0.5, 0.5, -0.3, 0.2);

        assert!(
            blobs[0].vx > 0.0,
            "Blob inside stir radius must gain positive vx"
        );
        assert!(
            blobs[0].vy < 0.0,
            "Blob inside stir radius must gain negative vy"
        );
        assert_eq!(
            blobs[1].vx, 0.0,
            "Blob outside stir radius must remain unaffected"
        );
        assert_eq!(
            blobs[1].vy, 0.0,
            "Blob outside stir radius must remain unaffected"
        );
    }

    #[test]
    fn test_ripple_injects_bounded_perturbation() {
        let mut blobs = vec![Blob::new(0.5, 0.5, 0.1, 0.5)];
        apply_ripple(&mut blobs, 2.0, || 1.0);

        assert!(blobs[0].vx > 0.0);
        assert!(blobs[0].vy > 0.0);
        assert!(blobs[0].vx <= 2.0);
        assert!(blobs[0].vy <= 2.0);
    }

    #[test]
    fn test_pressure_modulates_buoyancy() {
        let mut params = PhysicsParams::default();
        let initial_buoyancy = params.buoyancy;

        apply_pressure(&mut params, 1.0);
        assert!(params.buoyancy > initial_buoyancy);

        apply_pressure(&mut params, -2.0);
        assert!(params.buoyancy < initial_buoyancy);
    }

    #[test]
    fn test_thermal_pulse_modulates_temperature() {
        let mut blobs = vec![Blob::new(0.5, 0.5, 0.1, 0.3), Blob::new(0.9, 0.9, 0.1, 0.3)];

        apply_thermal_pulse(&mut blobs, 0.5, 0.5, 0.5, 0.2);
        assert!(blobs[0].temperature > 0.3);
        assert_eq!(blobs[1].temperature, 0.3);
    }
}
