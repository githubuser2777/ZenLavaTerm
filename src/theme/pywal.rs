//! Pywal theme extractor for Linux ricing environments.

use crate::render::{ColorPalette, Rgb};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Pywal JSON schema representation.
#[derive(Debug, Deserialize)]
struct PywalJson {
    special: Option<PywalSpecial>,
    colors: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct PywalSpecial {
    background: Option<String>,
    #[allow(dead_code)]
    foreground: Option<String>,
}

/// Parses pywal `colors.json` string into a `ColorPalette`.
pub fn parse_pywal_json(json_str: &str) -> Result<ColorPalette, String> {
    let pywal: PywalJson =
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse Pywal JSON: {e}"))?;

    let default_palette = ColorPalette::default();

    let bg = pywal
        .special
        .as_ref()
        .and_then(|s| s.background.as_deref())
        .and_then(|h| Rgb::from_hex(h).ok())
        .or_else(|| {
            pywal
                .colors
                .as_ref()
                .and_then(|c| c.get("color0"))
                .and_then(|h| Rgb::from_hex(h).ok())
        })
        .unwrap_or(default_palette.background);

    let colors = pywal.colors.unwrap_or_default();

    let bottom = colors
        .get("color1")
        .or_else(|| colors.get("color9"))
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_palette.bottom);

    let middle = colors
        .get("color3")
        .or_else(|| colors.get("color11"))
        .or_else(|| colors.get("color2"))
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_palette.middle);

    let top = colors
        .get("color4")
        .or_else(|| colors.get("color12"))
        .or_else(|| colors.get("color5"))
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_palette.top);

    Ok(ColorPalette {
        bottom,
        middle,
        top,
        background: bg,
    })
}

/// Parses pywal flat `colors` plain-text file (16 lines of `#rrggbb`).
pub fn parse_pywal_flat(text: &str) -> Result<ColorPalette, String> {
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') || l.len() >= 6)
        .collect();

    if lines.is_empty() {
        return Err("Pywal flat file is empty".to_string());
    }

    let default_pal = ColorPalette::default();
    let bg = lines
        .first()
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_pal.background);
    let bottom = lines
        .get(1)
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_pal.bottom);
    let middle = lines
        .get(3)
        .or_else(|| lines.get(2))
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_pal.middle);
    let top = lines
        .get(4)
        .or_else(|| lines.get(5))
        .and_then(|h| Rgb::from_hex(h).ok())
        .unwrap_or(default_pal.top);

    Ok(ColorPalette {
        bottom,
        middle,
        top,
        background: bg,
    })
}

/// Returns the standard default Pywal cache paths in order of preference across platforms.
pub fn default_pywal_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
        let p = PathBuf::from(cache_home).join("wal");
        paths.push(p.join("colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(".cache").join("wal");
        paths.push(p.join("colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        let p = PathBuf::from(local_appdata).join("wal");
        paths.push(p.join("colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        let p = PathBuf::from(userprofile).join(".cache").join("wal");
        paths.push(p.join("colors.json"));
        paths.push(p.join("colors"));
    }
    paths
}

/// Attempts to load Pywal colors from a specific path.
pub fn load_pywal_from_path(path: &Path) -> Result<ColorPalette, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read pywal cache file at {}: {e}", path.display()))?;

    if path.extension().and_then(|s| s.to_str()) == Some("json")
        || content.trim_start().starts_with('{')
    {
        parse_pywal_json(&content)
    } else {
        parse_pywal_flat(&content)
    }
}

/// Attempts to load Pywal colors from default cached locations.
pub fn load_pywal_default() -> Result<ColorPalette, String> {
    for path in default_pywal_paths() {
        if path.exists() {
            if let Ok(palette) = load_pywal_from_path(&path) {
                return Ok(palette);
            }
        }
    }
    Err("No valid Pywal cache found in default locations (~/.cache/wal/)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pywal_json_full() {
        let sample = r##"{
            "special": {
                "background": "#1e1e2e",
                "foreground": "#cdd6f4"
            },
            "colors": {
                "color0": "#1e1e2e",
                "color1": "#f38ba8",
                "color2": "#a6e3a1",
                "color3": "#f9e2af",
                "color4": "#89b4fa",
                "color5": "#f5c2e7"
            }
        }"##;

        let palette = parse_pywal_json(sample).expect("Parsed pywal JSON");
        assert_eq!(palette.background, Rgb::new(0x1E, 0x1E, 0x2E));
        assert_eq!(palette.bottom, Rgb::new(0xF3, 0x8B, 0xA8));
        assert_eq!(palette.middle, Rgb::new(0xF9, 0xE2, 0xAF));
        assert_eq!(palette.top, Rgb::new(0x89, 0xB4, 0xFA));
    }

    #[test]
    fn test_parse_pywal_flat() {
        let flat = "#1a1b26\n#f7768e\n#9ece6a\n#e0af68\n#7aa2f7\n#bb9af7\n";
        let palette = parse_pywal_flat(flat).expect("Parsed pywal flat");
        assert_eq!(palette.background, Rgb::new(0x1A, 0x1B, 0x26));
        assert_eq!(palette.bottom, Rgb::new(0xF7, 0x76, 0x8E));
        assert_eq!(palette.middle, Rgb::new(0xE0, 0xAF, 0x68));
        assert_eq!(palette.top, Rgb::new(0x7A, 0xA2, 0xF7));
    }
}
