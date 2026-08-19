//! Wallust theme extractor for modern Linux dynamic color schemes.

use crate::render::{ColorPalette, Rgb};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Wallust JSON schema supporting diverse template outputs.
#[derive(Debug, Deserialize)]
struct WallustJson {
    background: Option<String>,
    color0: Option<String>,
    color1: Option<String>,
    color2: Option<String>,
    color3: Option<String>,
    color4: Option<String>,
    color5: Option<String>,
    color9: Option<String>,
    color11: Option<String>,
    color12: Option<String>,
    colors: Option<HashMap<String, String>>,
    palette: Option<HashMap<String, String>>,
}

/// Parses Wallust JSON string into a `ColorPalette`.
pub fn parse_wallust_json(json_str: &str) -> Result<ColorPalette, String> {
    let wallust: WallustJson =
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse Wallust JSON: {e}"))?;

    let default_palette = ColorPalette::default();

    let bg = wallust
        .background
        .as_deref()
        .or(wallust.color0.as_deref())
        .and_then(|h| Rgb::from_hex(h).ok())
        .or_else(|| {
            wallust
                .colors
                .as_ref()
                .and_then(|c| c.get("color0").or_else(|| c.get("background")))
                .and_then(|h| Rgb::from_hex(h).ok())
        })
        .or_else(|| {
            wallust
                .palette
                .as_ref()
                .and_then(|c| c.get("color0").or_else(|| c.get("background")))
                .and_then(|h| Rgb::from_hex(h).ok())
        })
        .unwrap_or(default_palette.background);

    let get_color = |key1: &str, key2: &str| -> Option<Rgb> {
        let direct = match key1 {
            "color1" => wallust.color1.as_deref(),
            "color2" => wallust.color2.as_deref(),
            "color3" => wallust.color3.as_deref(),
            "color4" => wallust.color4.as_deref(),
            "color5" => wallust.color5.as_deref(),
            _ => None,
        };
        direct
            .and_then(|h| Rgb::from_hex(h).ok())
            .or_else(|| {
                let direct2 = match key2 {
                    "color9" => wallust.color9.as_deref(),
                    "color11" => wallust.color11.as_deref(),
                    "color12" => wallust.color12.as_deref(),
                    _ => None,
                };
                direct2.and_then(|h| Rgb::from_hex(h).ok())
            })
            .or_else(|| {
                wallust
                    .colors
                    .as_ref()
                    .and_then(|c| c.get(key1).or_else(|| c.get(key2)))
                    .and_then(|h| Rgb::from_hex(h).ok())
            })
            .or_else(|| {
                wallust
                    .palette
                    .as_ref()
                    .and_then(|c| c.get(key1).or_else(|| c.get(key2)))
                    .and_then(|h| Rgb::from_hex(h).ok())
            })
    };

    let bottom = get_color("color1", "color9").unwrap_or(default_palette.bottom);
    let middle = get_color("color3", "color11")
        .or_else(|| get_color("color2", "color10"))
        .unwrap_or(default_palette.middle);
    let top = get_color("color4", "color12")
        .or_else(|| get_color("color5", "color13"))
        .unwrap_or(default_palette.top);

    Ok(ColorPalette {
        bottom,
        middle,
        top,
        background: bg,
    })
}

/// Returns the standard default Wallust cache paths in order of preference across platforms.
pub fn default_wallust_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
        let p = PathBuf::from(cache_home).join("wallust");
        paths.push(p.join("colors.json"));
        paths.push(p.join("nix-colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(".cache").join("wallust");
        paths.push(p.join("colors.json"));
        paths.push(p.join("nix-colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        let p = PathBuf::from(local_appdata).join("wallust");
        paths.push(p.join("colors.json"));
        paths.push(p.join("nix-colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(appdata) = std::env::var("APPDATA") {
        let p = PathBuf::from(appdata).join("wallust");
        paths.push(p.join("colors.json"));
        paths.push(p.join("nix-colors.json"));
        paths.push(p.join("colors"));
    }
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        let p = PathBuf::from(userprofile).join(".cache").join("wallust");
        paths.push(p.join("colors.json"));
        paths.push(p.join("nix-colors.json"));
        paths.push(p.join("colors"));
    }
    paths
}

/// Attempts to load Wallust colors from a specific path.
pub fn load_wallust_from_path(path: &Path) -> Result<ColorPalette, String> {
    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read wallust cache file at {}: {e}",
            path.display()
        )
    })?;

    if path.extension().and_then(|s| s.to_str()) == Some("json")
        || content.trim_start().starts_with('{')
    {
        parse_wallust_json(&content)
    } else {
        super::pywal::parse_pywal_flat(&content)
    }
}

/// Attempts to load Wallust colors from default cached locations.
pub fn load_wallust_default() -> Result<ColorPalette, String> {
    for path in default_wallust_paths() {
        if path.exists() {
            if let Ok(palette) = load_wallust_from_path(&path) {
                return Ok(palette);
            }
        }
    }
    Err("No valid Wallust cache found in default locations (~/.cache/wallust/)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wallust_json_flat_format() {
        let sample = r##"{
            "background": "#0f141c",
            "color0": "#0f141c",
            "color1": "#e06c75",
            "color2": "#98c379",
            "color3": "#e5c07b",
            "color4": "#61afef",
            "color5": "#c678dd"
        }"##;

        let palette = parse_wallust_json(sample).expect("Parsed wallust flat JSON");
        assert_eq!(palette.background, Rgb::new(0x0F, 0x14, 0x1C));
        assert_eq!(palette.bottom, Rgb::new(0xE0, 0x6C, 0x75));
        assert_eq!(palette.middle, Rgb::new(0xE5, 0xC0, 0x7B));
        assert_eq!(palette.top, Rgb::new(0x61, 0xAF, 0xEF));
    }
}
