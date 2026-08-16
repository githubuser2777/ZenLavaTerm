//! Curated theme presets and palette mappings.

use crate::render::{ColorPalette, Rgb};

/// A named theme preset containing 4 gradient anchors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemePreset {
    pub name: &'static str,
    pub description: &'static str,
    pub bottom: Rgb,
    pub middle: Rgb,
    pub top: Rgb,
    pub background: Rgb,
}

impl ThemePreset {
    /// Converts the preset into a `ColorPalette`.
    pub fn to_palette(&self) -> ColorPalette {
        ColorPalette {
            bottom: self.bottom,
            middle: self.middle,
            top: self.top,
            background: self.background,
        }
    }
}

/// List of all built-in theme presets.
pub static PRESETS: &[ThemePreset] = &[
    ThemePreset {
        name: "lava",
        description: "Classic warm lava lamp glowing embers",
        bottom: Rgb::new(0xFF, 0x3B, 0x00), // #ff3b00 (fiery red-orange)
        middle: Rgb::new(0xFF, 0x7A, 0x00), // #ff7a00 (warm amber)
        top: Rgb::new(0x7B, 0x2C, 0xFF),    // #7b2cff (cool violet)
        background: Rgb::new(0x0D, 0x0D, 0x15), // #0d0d15 (dark space)
    },
    ThemePreset {
        name: "ocean",
        description: "Deep bioluminescent oceanic abyss",
        bottom: Rgb::new(0x00, 0xF0, 0xFF), // #00f0ff (cyan core)
        middle: Rgb::new(0x00, 0x77, 0xBE), // #0077be (ocean blue)
        top: Rgb::new(0x0A, 0x19, 0x2F),    // #0a192f (midnight abyss)
        background: Rgb::new(0x02, 0x0B, 0x14), // #020b14 (deep trench)
    },
    ThemePreset {
        name: "cyberpunk",
        description: "High-contrast neon yellow and hot magenta",
        bottom: Rgb::new(0xFC, 0xEE, 0x0A), // #fcee0a (neon yellow)
        middle: Rgb::new(0xFF, 0x00, 0x55), // #ff0055 (hot magenta)
        top: Rgb::new(0x71, 0x22, 0xFA),    // #7122fa (electric purple)
        background: Rgb::new(0x05, 0x05, 0x0D), // #05050d (night city)
    },
    ThemePreset {
        name: "synthwave",
        description: "80s retrowave outrun sunset glow",
        bottom: Rgb::new(0xFF, 0x2A, 0x85), // #ff2a85 (neon pink)
        middle: Rgb::new(0x9A, 0x48, 0xD0), // #9a48d0 (retro purple)
        top: Rgb::new(0x2D, 0xE2, 0xE6),    // #2de2e6 (laser cyan)
        background: Rgb::new(0x12, 0x0B, 0x22), // #120b22 (deep twilight)
    },
    ThemePreset {
        name: "nord",
        description: "Arctic pastel blues and frosty ice",
        bottom: Rgb::new(0x88, 0xC0, 0xD0), // #88c0d0 (frost ice)
        middle: Rgb::new(0x5E, 0x81, 0xAC), // #5e81ac (nordic blue)
        top: Rgb::new(0x81, 0xA1, 0xC1),    // #81a1c1 (polar mist)
        background: Rgb::new(0x2E, 0x34, 0x40), // #2e3440 (nord dark night)
    },
    ThemePreset {
        name: "forest",
        description: "Emerald greens and earthy woodland glow",
        bottom: Rgb::new(0x55, 0xFF, 0x77), // #55ff77 (bright moss)
        middle: Rgb::new(0x2E, 0x8B, 0x57), // #2e8b57 (sea green)
        top: Rgb::new(0x1B, 0x43, 0x32),    // #1b4332 (deep pine)
        background: Rgb::new(0x08, 0x1C, 0x15), // #081c15 (canopy shadow)
    },
    ThemePreset {
        name: "monochrome",
        description: "High-contrast minimalist grayscale",
        bottom: Rgb::new(0xFF, 0xFF, 0xFF), // #ffffff (pure white)
        middle: Rgb::new(0x99, 0x99, 0x99), // #999999 (neutral gray)
        top: Rgb::new(0x44, 0x44, 0x44),    // #444444 (dark gray)
        background: Rgb::new(0x0A, 0x0A, 0x0A), // #0a0a0a (charcoal black)
    },
    ThemePreset {
        name: "matrix",
        description: "Terminal phosphor matrix green cascade",
        bottom: Rgb::new(0xA6, 0xFF, 0x00), // #a6ff00 (acid lime)
        middle: Rgb::new(0x00, 0xFF, 0x41), // #00ff41 (matrix green)
        top: Rgb::new(0x00, 0x3B, 0x00),    // #003b00 (dark phosphor)
        background: Rgb::new(0x0D, 0x11, 0x17), // #0d1117 (mainframe black)
    },
    ThemePreset {
        name: "sunset",
        description: "Dusk twilight with blazing orange and royal purple",
        bottom: Rgb::new(0xFF, 0x45, 0x00), // #ff4500 (orange-red)
        middle: Rgb::new(0xFF, 0x8C, 0x00), // #ff8c00 (dark orange)
        top: Rgb::new(0x4A, 0x0E, 0x4E),    // #4a0e4e (royal plum)
        background: Rgb::new(0x1A, 0x00, 0x22), // #1a0022 (dusk background)
    },
    ThemePreset {
        name: "dracula",
        description: "Gothic vampire pink, purple and cyan accents",
        bottom: Rgb::new(0xFF, 0x79, 0xC6), // #ff79c6 (dracula pink)
        middle: Rgb::new(0xBD, 0x93, 0xF9), // #bd93f9 (dracula purple)
        top: Rgb::new(0x8B, 0xE9, 0xFD),    // #8be9fd (dracula cyan)
        background: Rgb::new(0x28, 0x2A, 0x36), // #282a36 (dracula background)
    },
    ThemePreset {
        name: "catppuccin",
        description: "Catppuccin Mocha soothing pastel tones",
        bottom: Rgb::new(0xF5, 0xC2, 0xE7),     // #f5c2e7 (pink)
        middle: Rgb::new(0xCB, 0xA6, 0xF7),     // #cba6f7 (mauve)
        top: Rgb::new(0x89, 0xB4, 0xFA),        // #89b4fa (blue)
        background: Rgb::new(0x1E, 0x1E, 0x2E), // #1e1e2e (crust base)
    },
    ThemePreset {
        name: "tokyo-night",
        description: "Tokyo Night neon glow in metropolitan rain",
        bottom: Rgb::new(0xF7, 0x76, 0x8E), // #f7768e (red/pink)
        middle: Rgb::new(0xBB, 0x9A, 0xF7), // #bb9af7 (purple)
        top: Rgb::new(0x7A, 0xA2, 0xF7),    // #7aa2f7 (blue)
        background: Rgb::new(0x1A, 0x1B, 0x26), // #1a1b26 (night storm)
    },
];

/// Returns the `ColorPalette` corresponding to a named preset if found.
pub fn get_preset_palette(name: &str) -> Option<ColorPalette> {
    let normalized = name.trim().to_lowercase().replace('_', "-");
    PRESETS
        .iter()
        .find(|p| {
            p.name == normalized
                || (p.name == "lava" && (normalized == "classic" || normalized == "default"))
        })
        .map(|p| p.to_palette())
}

/// Returns a slice of all available preset theme names.
pub fn list_presets() -> Vec<&'static str> {
    PRESETS.iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets_valid() {
        for preset in PRESETS {
            assert!(!preset.name.is_empty());
            let pal = preset.to_palette();
            assert_ne!(pal.bottom, pal.background);
        }
    }

    #[test]
    fn test_preset_lookup_case_insensitive() {
        assert!(get_preset_palette("OCEAN").is_some());
        assert!(get_preset_palette("cyberpunk").is_some());
        assert!(get_preset_palette("tokyo_night").is_some());
        assert!(get_preset_palette("tokyo-night").is_some());
        assert!(get_preset_palette("lava").is_some());
        assert!(get_preset_palette("classic").is_some());
        assert!(get_preset_palette("nonexistent_preset").is_none());
    }
}
