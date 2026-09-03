//! Theme resolution engine.

use super::{
    detector::detect_auto_theme,
    file::load_custom_theme_file,
    preset::{get_preset_palette, list_presets},
    pywal::load_pywal_default,
    wallust::load_wallust_default,
};
use crate::render::ColorPalette;
use std::path::Path;

/// Resolves a theme specification string into a `ColorPalette`.
///
/// `spec` can be:
/// - `"auto"`: Auto-detects from Pywal, Wallust, or default.
/// - `"pywal"`: Reads from standard Pywal cache.
/// - `"wallust"`: Reads from standard Wallust cache.
/// - Preset name: `"ocean"`, `"cyberpunk"`, `"synthwave"`, `"nord"`, `"forest"`, `"monochrome"`, `"matrix"`, etc.
/// - File path: `/path/to/theme.json`, `./custom.toml`, etc.
pub fn resolve_theme(spec: &str) -> Result<ColorPalette, String> {
    let clean = spec.trim();

    if clean.eq_ignore_ascii_case("auto") {
        return Ok(detect_auto_theme());
    }

    if clean.eq_ignore_ascii_case("pywal") {
        return match load_pywal_default() {
            Ok(p) => Ok(p),
            Err(e) => {
                eprintln!("Warning: {e}. Falling back to default lava theme.");
                Ok(get_preset_palette("lava").unwrap_or_default())
            }
        };
    }

    if clean.eq_ignore_ascii_case("wallust") {
        return match load_wallust_default() {
            Ok(p) => Ok(p),
            Err(e) => {
                eprintln!("Warning: {e}. Falling back to default lava theme.");
                Ok(get_preset_palette("lava").unwrap_or_default())
            }
        };
    }

    if let Some(pal) = get_preset_palette(clean) {
        return Ok(pal);
    }

    let path = Path::new(clean);
    if path.exists()
        || clean.ends_with(".json")
        || clean.ends_with(".toml")
        || clean.contains('/')
        || clean.contains('\\')
    {
        return load_custom_theme_file(path);
    }

    Err(format!(
        "Invalid theme '{}'. Must be 'auto', 'pywal', 'wallust', a file path, or one of the presets: {}",
        clean,
        list_presets().join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_presets() {
        assert!(resolve_theme("ocean").is_ok());
        assert!(resolve_theme("cyberpunk").is_ok());
        assert!(resolve_theme("synthwave").is_ok());
        assert!(resolve_theme("nord").is_ok());
        assert!(resolve_theme("matrix").is_ok());
    }

    #[test]
    fn test_resolve_auto() {
        assert!(resolve_theme("auto").is_ok());
    }

    #[test]
    fn test_resolve_invalid_name() {
        let err = resolve_theme("invalid_theme_name_12345").unwrap_err();
        assert!(err.contains("Invalid theme"));
        assert!(err.contains("presets:"));
    }
}
