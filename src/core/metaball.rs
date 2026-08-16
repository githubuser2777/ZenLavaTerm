//! Blob data model representing an individual fluid metaball.

/// Represents an individual metaball in normalized simulation space $[0.0, 1.0] \times [0.0, 1.0]$.
#[derive(Debug, Clone, PartialEq)]
pub struct Blob {
    /// Horizontal position in $[0.0, 1.0]$.
    pub x: f32,
    /// Vertical position in $[0.0, 1.0]$ ($0.0$ = bottom heat source, $1.0$ = top cooling zone).
    pub y: f32,
    /// Horizontal velocity.
    pub vx: f32,
    /// Vertical velocity.
    pub vy: f32,
    /// Influence radius of the metaball.
    pub radius: f32,
    /// Thermal state in $[0.0, 1.0]$ ($1.0$ is hottest, rising; $0.0$ is coldest, sinking).
    pub temperature: f32,
}

impl Blob {
    /// Creates a new `Blob` with validated coordinates.
    pub fn new(x: f32, y: f32, radius: f32, temperature: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            vx: 0.0,
            vy: 0.0,
            radius: radius.max(0.01),
            temperature: temperature.clamp(0.0, 1.0),
        }
    }

    /// Squared Euclidean distance from this blob to a point `(px, py)`.
    #[inline]
    pub fn distance_sq_to(&self, px: f32, py: f32) -> f32 {
        let dx = self.x - px;
        let dy = self.y - py;
        dx * dx + dy * dy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_creation_and_clamping() {
        let blob = Blob::new(1.5, -0.2, -5.0, 2.0);
        assert_eq!(blob.x, 1.0);
        assert_eq!(blob.y, 0.0);
        assert!(blob.radius >= 0.01);
        assert_eq!(blob.temperature, 1.0);
    }

    #[test]
    fn test_distance_squared() {
        let blob = Blob::new(0.5, 0.5, 0.1, 0.5);
        let d_sq = blob.distance_sq_to(0.5, 0.8);
        assert!((d_sq - 0.09).abs() < 1e-5);
    }
}
