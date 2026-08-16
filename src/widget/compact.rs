//! Adaptive geometry evaluation and compact physics profile scaling for small viewports.

use crate::core::PhysicsParams;

/// Checks if compact layout and scaling policy should be activated.
///
/// Automatic activation triggers when terminal geometry is constrained (`cols < 40` or `rows < 15`),
/// or when explicitly requested via CLI / config.
#[inline]
pub fn should_compact(cols: u16, rows: u16, explicit_compact: bool) -> bool {
    explicit_compact || cols < 40 || rows < 15
}

/// Scaled physical parameters adapted for compact viewports.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompactProfile {
    /// Scaled blob count to prevent visual saturation.
    pub blob_count: usize,
    /// Multiplier for blob radii.
    pub radius_scale: f32,
    /// Multiplier for convective buoyancy.
    pub buoyancy_scale: f32,
    /// Multiplier for Brownian thermal noise.
    pub noise_scale: f32,
}

/// Scaler calculating compact simulation profiles deterministically from viewport geometry.
pub struct CompactScaler;

impl CompactScaler {
    /// Computes the adapted compact profile deterministically based on viewport geometry.
    pub fn calculate_profile(cols: u16, rows: u16, base_blobs: usize) -> CompactProfile {
        let area = (cols as usize) * (rows as usize);

        if area < 200 {
            // Micro viewport: e.g. 10x3, 15x5, 20x8
            CompactProfile {
                blob_count: base_blobs.clamp(2, 4),
                radius_scale: 0.65,
                buoyancy_scale: 1.25,
                noise_scale: 0.80,
            }
        } else if area < 800 {
            // Small compact viewport: e.g. 24x8, 40x15
            CompactProfile {
                blob_count: base_blobs.clamp(4, 8),
                radius_scale: 0.85,
                buoyancy_scale: 1.10,
                noise_scale: 0.90,
            }
        } else {
            // Standard / large viewport: e.g. 80x24, 200x60
            CompactProfile {
                blob_count: base_blobs,
                radius_scale: 1.0,
                buoyancy_scale: 1.0,
                noise_scale: 1.0,
            }
        }
    }

    /// Adapts base physics parameters using the compact profile.
    pub fn adapt_physics(profile: &CompactProfile, base: PhysicsParams) -> PhysicsParams {
        PhysicsParams {
            gravity: base.gravity,
            buoyancy: base.buoyancy * profile.buoyancy_scale,
            viscosity: base.viscosity,
            noise: base.noise * profile.noise_scale,
            thermal_transfer_rate: base.thermal_transfer_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_compact_rules() {
        // Automatic activation on small columns or rows
        assert!(should_compact(39, 24, false));
        assert!(should_compact(80, 14, false));
        assert!(should_compact(20, 8, false));

        // Normal large viewport
        assert!(!should_compact(80, 24, false));
        assert!(!should_compact(120, 40, false));

        // Explicit override forces compact on large viewports
        assert!(should_compact(80, 24, true));
        assert!(should_compact(200, 60, true));
    }

    #[test]
    fn test_calculate_profile_geometry_matrix() {
        let base_blobs = 12;

        // 1. Micro viewports: 10x3 (area 30), 15x5 (area 75), 20x8 (area 160)
        for (cols, rows) in [(10, 3), (15, 5), (20, 8)] {
            let profile = CompactScaler::calculate_profile(cols, rows, base_blobs);
            assert!(profile.blob_count <= 4);
            assert!(profile.blob_count >= 2);
            assert_eq!(profile.radius_scale, 0.65);
            assert_eq!(profile.buoyancy_scale, 1.25);
            assert_eq!(profile.noise_scale, 0.80);
        }

        // 2. Small compact viewports: 24x8 (area 192 -> micro, wait 24*8 = 192 < 200 so micro; let's check 40x15 = 600)
        let profile_40x15 = CompactScaler::calculate_profile(40, 15, base_blobs);
        assert_eq!(profile_40x15.blob_count, 8);
        assert_eq!(profile_40x15.radius_scale, 0.85);
        assert_eq!(profile_40x15.buoyancy_scale, 1.10);
        assert_eq!(profile_40x15.noise_scale, 0.90);

        // 3. Standard / large viewports: 80x24 (area 1920), 200x60 (area 12000)
        for (cols, rows) in [(80, 24), (200, 60)] {
            let profile = CompactScaler::calculate_profile(cols, rows, base_blobs);
            assert_eq!(profile.blob_count, base_blobs);
            assert_eq!(profile.radius_scale, 1.0);
            assert_eq!(profile.buoyancy_scale, 1.0);
            assert_eq!(profile.noise_scale, 1.0);
        }
    }

    #[test]
    fn test_adapt_physics_parameters() {
        let base = PhysicsParams {
            gravity: 0.12,
            buoyancy: 0.80,
            viscosity: 0.93,
            noise: 0.15,
            thermal_transfer_rate: 0.40,
        };

        let profile = CompactScaler::calculate_profile(20, 8, 12);
        let adapted = CompactScaler::adapt_physics(&profile, base);

        assert_eq!(adapted.gravity, 0.12);
        assert_eq!(adapted.buoyancy, 0.80 * 1.25);
        assert_eq!(adapted.viscosity, 0.93);
        assert_eq!(adapted.noise, 0.15 * 0.80);
        assert_eq!(adapted.thermal_transfer_rate, 0.40);
    }
}
