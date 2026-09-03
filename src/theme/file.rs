//! Custom theme file parser supporting JSON, TOML, and 16-color schemes.

use crate::render::{ColorPalette, Rgb};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct DirectPaletteSchema {
    bottom: Option<String>,
    middle: Option<String>,
    top: Option<String>,
    background: Option<String>,
}

impl DirectPaletteSchema {
    fn to_palette(&self) -> Option<ColorPalette> {
        if self.bottom.is_some() || self.top.is_some() {
            let default_pal = ColorPalette::default();
            Some(ColorPalette {
                bottom: self
                    .bottom
                    .as_deref()
                    .and_then(|h| Rgb::from_hex(h).ok())
                    .unwrap_or(default_pal.bottom),
                middle: self
                    .middle
                    .as_deref()
                    .and_then(|h| Rgb::from_hex(h).ok())
                    .unwrap_or(default_pal.middle),
                top: self
                    .top
                    .as_deref()
                    .and_then(|h| Rgb::from_hex(h).ok())
                    .unwrap_or(default_pal.top),
                background: self
                    .background
                    .as_deref()
                    .and_then(|h| Rgb::from_hex(h).ok())
                    .unwrap_or(default_pal.background),
            })
        } else {
            None
        }
    }
}

/// Loads and parses a custom theme file from any arbitrary path.
pub fn load_custom_theme_file(path: &Path) -> Result<ColorPalette, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read theme file at '{}': {e}", path.display()))?;

    let trimmed = content.trim();

    // 1. Try parsing as TOML direct palette schema
    if let Ok(toml_palette) = toml::from_str::<DirectPaletteSchema>(trimmed) {
        if let Some(pal) = toml_palette.to_palette() {
            return Ok(pal);
        }
    }

    // 2. Try parsing as JSON direct palette schema
    if let Ok(json_palette) = serde_json::from_str::<DirectPaletteSchema>(trimmed) {
        if let Some(pal) = json_palette.to_palette() {
            return Ok(pal);
        }
    }

    // 3. Try parsing as Pywal/Wallust JSON
    if trimmed.starts_with('{') {
        if let Ok(pal) = super::pywal::parse_pywal_json(trimmed) {
            return Ok(pal);
        }
        if let Ok(pal) = super::wallust::parse_wallust_json(trimmed) {
            return Ok(pal);
        }
    }

    // 4. Try parsing as flat 16-color hex text
    super::pywal::parse_pywal_flat(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direct_json() {
        let json = r##"{
            "bottom": "#00ffcc",
            "middle": "#0077ff",
            "top": "#ff0077",
            "background": "#000011"
        }"##;
        let pal = serde_json::from_str::<DirectPaletteSchema>(json).unwrap();
        let b = Rgb::from_hex(pal.bottom.as_deref().unwrap()).unwrap();
        assert_eq!(b, Rgb::new(0x00, 0xFF, 0xCC));
    }
}
