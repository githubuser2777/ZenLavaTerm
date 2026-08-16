//! Physics and thermodynamic convection models for metaball particles.

use super::metaball::Blob;

/// Parameters governing fluid mechanics and thermal convection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsParams {
    /// Gravitational downward acceleration constant.
    pub gravity: f32,
    /// Upward buoyancy acceleration multiplier for hot fluid.
    pub buoyancy: f32,
    /// Fluid viscosity drag damping factor (range: $0.0..1.0$).
    pub viscosity: f32,
    /// Brownian thermal noise perturbation amplitude.
    pub noise: f32,
    /// Rate of thermal exchange with the bottom and top boundaries.
    pub thermal_transfer_rate: f32,
}

impl Default for PhysicsParams {
    fn default() -> Self {
        Self {
            gravity: 0.12,
            buoyancy: 0.80,
            viscosity: 0.93,
            noise: 0.15,
            thermal_transfer_rate: 0.40,
        }
    }
}

/// Applies forces, thermodynamic convection, and numerical integration to a blob.
pub fn step_blob(blob: &mut Blob, params: &PhysicsParams, dt: f32, noise_offset: (f32, f32)) {
    // 1. Thermodynamic Convection:
    // Heat up near the bottom plate (y < 0.25)
    if blob.y < 0.25 {
        let heat_factor = (0.25 - blob.y) / 0.25;
        blob.temperature =
            (blob.temperature + params.thermal_transfer_rate * heat_factor * dt).min(1.0);
    }
    // Cool down near the top surface (y > 0.75)
    if blob.y > 0.75 {
        let cool_factor = (blob.y - 0.75) / 0.25;
        blob.temperature =
            (blob.temperature - params.thermal_transfer_rate * cool_factor * dt).max(0.0);
    }

    // 2. Net Vertical Force:
    // Buoyancy accelerates hot blobs upwards, gravity pulls down
    let buoyancy_force = params.buoyancy * (blob.temperature - 0.5);
    let net_ay = buoyancy_force - params.gravity;

    // 3. Apply Accelerations & Noise:
    blob.vy += net_ay * dt + noise_offset.1 * params.noise * dt;
    blob.vx += noise_offset.0 * params.noise * dt;

    // 4. Viscous Drag Damping:
    let drag = (1.0 - (1.0 - params.viscosity) * dt).clamp(0.0, 1.0);
    blob.vx *= drag;
    blob.vy *= drag;

    // 5. Integrate Position:
    blob.x += blob.vx * dt;
    blob.y += blob.vy * dt;

    // 6. Boundary Handling (Soft elastic reflection):
    let margin = 0.02;
    if blob.x < margin {
        blob.x = margin;
        blob.vx = -blob.vx * 0.5;
    } else if blob.x > 1.0 - margin {
        blob.x = 1.0 - margin;
        blob.vx = -blob.vx * 0.5;
    }

    if blob.y < margin {
        blob.y = margin;
        blob.vy = -blob.vy * 0.3;
    } else if blob.y > 1.0 - margin {
        blob.y = 1.0 - margin;
        blob.vy = -blob.vy * 0.3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_blob_rises() {
        let mut hot_blob = Blob::new(0.5, 0.2, 0.1, 1.0);
        let params = PhysicsParams {
            buoyancy: 1.0,
            gravity: 0.1,
            viscosity: 0.95,
            noise: 0.0,
            thermal_transfer_rate: 0.0,
        };

        let initial_y = hot_blob.y;
        step_blob(&mut hot_blob, &params, 0.1, (0.0, 0.0));
        assert!(
            hot_blob.y > initial_y,
            "Hot blob should rise due to buoyancy"
        );
    }

    #[test]
    fn test_cold_blob_sinks() {
        let mut cold_blob = Blob::new(0.5, 0.8, 0.1, 0.0);
        let params = PhysicsParams {
            buoyancy: 1.0,
            gravity: 0.1,
            viscosity: 0.95,
            noise: 0.0,
            thermal_transfer_rate: 0.0,
        };

        let initial_y = cold_blob.y;
        step_blob(&mut cold_blob, &params, 0.1, (0.0, 0.0));
        assert!(
            cold_blob.y < initial_y,
            "Cold blob should sink under gravity"
        );
    }
}
