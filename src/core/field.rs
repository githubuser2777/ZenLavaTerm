//! 2D Scalar field evaluation and isosurface potential calculation.

use super::metaball::Blob;

/// Small epsilon to prevent division by zero in field potential calculations.
const EPSILON: f32 = 1e-4;

/// Computes continuous scalar field potential and temperature distribution from blobs.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScalarField;

impl ScalarField {
    /// Evaluates the total scalar field intensity at coordinate `(px, py)`.
    ///
    /// $F(px, py) = \sum_{i} \frac{R_i^2}{d_i^2 + \epsilon}$
    #[inline]
    pub fn evaluate_field(&self, blobs: &[Blob], px: f32, py: f32) -> f32 {
        // ponytail: O(N*blobs) per-pixel scan; spatial partitioning/bounding boxes if blob count > 50
        let mut sum = 0.0;
        for blob in blobs {
            let dx = px - blob.x;
            let dy = py - blob.y;
            let d_sq = dx * dx + dy * dy;
            let r_sq = blob.radius * blob.radius;
            sum += r_sq / (d_sq + EPSILON);
        }
        sum
    }

    /// Evaluates both the field intensity and the field-weighted temperature at `(px, py)`.
    ///
    /// Returns `(field_intensity, weighted_temperature)`.
    #[inline]
    pub fn evaluate_with_temperature(&self, blobs: &[Blob], px: f32, py: f32) -> (f32, f32) {
        let mut total_field = 0.0;
        let mut weighted_temp = 0.0;

        for blob in blobs {
            let dx = px - blob.x;
            let dy = py - blob.y;
            let d_sq = dx * dx + dy * dy;
            let r_sq = blob.radius * blob.radius;
            let contribution = r_sq / (d_sq + EPSILON);

            total_field += contribution;
            weighted_temp += contribution * blob.temperature;
        }

        let avg_temp = if total_field > 0.0 {
            weighted_temp / total_field
        } else {
            0.5
        };

        (total_field, avg_temp)
    }

    /// Determines if a given point exceeds the isosurface threshold.
    #[inline]
    pub fn is_inside(&self, blobs: &[Blob], px: f32, py: f32, threshold: f32) -> bool {
        self.evaluate_field(blobs, px, py) >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_decreases_with_distance() {
        let field = ScalarField;
        let blobs = vec![Blob::new(0.5, 0.5, 0.1, 1.0)];

        let near_val = field.evaluate_field(&blobs, 0.5, 0.55);
        let far_val = field.evaluate_field(&blobs, 0.5, 0.90);

        assert!(
            near_val > far_val,
            "Field must decrease as distance increases"
        );
    }

    #[test]
    fn test_field_superposition() {
        let field = ScalarField;
        let blob1 = Blob::new(0.4, 0.5, 0.1, 1.0);
        let blob2 = Blob::new(0.6, 0.5, 0.1, 1.0);

        let single_val = field.evaluate_field(std::slice::from_ref(&blob1), 0.5, 0.5);
        let dual_val = field.evaluate_field(&[blob1, blob2], 0.5, 0.5);

        assert!(
            dual_val > single_val * 1.5,
            "Two nearby blobs must superimpose their scalar fields"
        );
    }
}
