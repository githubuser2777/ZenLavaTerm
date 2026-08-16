//! 24-bit True Color representation, linear interpolation, and multi-stop palettes.

use crate::ParseColorError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 24-bit RGB Color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Creates a new RGB color.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parses an RGB color from a hex string (e.g. `"#ff3b00"` or `"ff3b00"`).
    pub fn from_hex(hex: &str) -> Result<Self, ParseColorError> {
        let clean_hex = hex.trim().trim_start_matches('#');
        if clean_hex.len() != 6 {
            return Err(ParseColorError(hex.to_string()));
        }

        let r = u8::from_str_radix(&clean_hex[0..2], 16)
            .map_err(|_| ParseColorError(hex.to_string()))?;
        let g = u8::from_str_radix(&clean_hex[2..4], 16)
            .map_err(|_| ParseColorError(hex.to_string()))?;
        let b = u8::from_str_radix(&clean_hex[4..6], 16)
            .map_err(|_| ParseColorError(hex.to_string()))?;

        Ok(Self::new(r, g, b))
    }

    /// Linearly interpolates between color `a` and `b` by factor `t` in $[0.0, 1.0]$.
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t_clamped = t.clamp(0.0, 1.0);
        let r = (a.r as f32 + (b.r as f32 - a.r as f32) * t_clamped).round() as u8;
        let g = (a.g as f32 + (b.g as f32 - a.g as f32) * t_clamped).round() as u8;
        let b_val = (a.b as f32 + (b.b as f32 - a.b as f32) * t_clamped).round() as u8;
        Self::new(r, g, b_val)
    }

    /// Formats the color as a hex string `#rrggbb`.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Rgb::from_hex(&s).map_err(serde::de::Error::custom)
    }
}

/// Palette holding gradient anchor colors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Heat source bottom color (hot).
    pub bottom: Rgb,
    /// Convective middle color.
    pub middle: Rgb,
    /// Cooled top chamber color (cold).
    pub top: Rgb,
    /// Fluid chamber background color.
    pub background: Rgb,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            bottom: Rgb::new(0xFF, 0x3B, 0x00),     // #ff3b00
            middle: Rgb::new(0xFF, 0x7A, 0x00),     // #ff7a00
            top: Rgb::new(0x7B, 0x2C, 0xFF),        // #7b2cff
            background: Rgb::new(0x0D, 0x0D, 0x15), // #0d0d15
        }
    }
}

impl ColorPalette {
    /// Samples the gradient based on normalized temperature and field intensity.
    pub fn sample_lava(&self, temp: f32, field_intensity: f32, threshold: f32) -> Rgb {
        if field_intensity < threshold {
            return self.background;
        }

        // Lava color based on temperature: [0.0 = top/cold] -> [0.5 = middle] -> [1.0 = bottom/hot]
        let t = temp.clamp(0.0, 1.0);
        let base_color = if t < 0.5 {
            Rgb::lerp(self.top, self.middle, t * 2.0)
        } else {
            Rgb::lerp(self.middle, self.bottom, (t - 0.5) * 2.0)
        };

        // Edge glow / rim highlights where field intensity is near threshold
        let edge_factor = ((field_intensity - threshold) / 0.3).clamp(0.0, 1.0);
        Rgb::lerp(Rgb::new(255, 255, 255), base_color, 0.4 + 0.6 * edge_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        let col = Rgb::from_hex("#ff3b00").expect("Valid hex");
        assert_eq!(col, Rgb::new(0xFF, 0x3B, 0x00));

        let col_no_hash = Rgb::from_hex("7b2cff").expect("Valid hex without hash");
        assert_eq!(col_no_hash, Rgb::new(0x7B, 0x2C, 0xFF));

        assert!(Rgb::from_hex("invalid").is_err());
        assert!(Rgb::from_hex("#123").is_err());
    }

    #[test]
    fn test_color_lerp() {
        let black = Rgb::new(0, 0, 0);
        let white = Rgb::new(200, 200, 200);

        let mid = Rgb::lerp(black, white, 0.5);
        assert_eq!(mid, Rgb::new(100, 100, 100));

        assert_eq!(Rgb::lerp(black, white, 0.0), black);
        assert_eq!(Rgb::lerp(black, white, 1.0), white);
    }
}
