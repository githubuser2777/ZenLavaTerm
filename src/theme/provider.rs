//! Theme provider trait abstraction and theme resolution engine.

use super::{
    detector::detect_auto_theme,
    file::load_custom_theme_file,
    preset::{get_preset_palette, list_presets},
    pywal::{load_pywal_default, load_pywal_from_path},
    wallust::{load_wallust_default, load_wallust_from_path},
};
use crate::render::ColorPalette;
use std::path::{Path, PathBuf};

/// Trait implemented by theme sources to produce a `ColorPalette`.
pub trait ThemeProvider {
    /// Loads or resolves the color palette.
    fn load_palette(&self) -> Result<ColorPalette, String>;
}

/// Provider for built-in named presets.
#[derive(Debug, Clone)]
pub struct PresetThemeProvider(pub String);

impl ThemeProvider for PresetThemeProvider {
    fn load_palette(&self) -> Result<ColorPalette, String> {
        get_preset_palette(&self.0).ok_or_else(|| {
            format!(
                "Unknown theme preset '{}'. Available presets: {}",
                self.0,
                list_presets().join(", ")
            )
        })
    }
}

/// Provider for Pywal desktop colors.
#[derive(Debug, Clone, Default)]
pub struct PywalThemeProvider {
    pub path: Option<PathBuf>,
}

impl ThemeProvider for PywalThemeProvider {
    fn load_palette(&self) -> Result<ColorPalette, String> {
        if let Some(ref p) = self.path {
            load_pywal_from_path(p)
        } else {
            load_pywal_default()
        }
    }
}

/// Provider for Wallust dynamic colors.
#[derive(Debug, Clone, Default)]
pub struct WallustThemeProvider {
    pub path: Option<PathBuf>,
}

impl ThemeProvider for WallustThemeProvider {
    fn load_palette(&self) -> Result<ColorPalette, String> {
        if let Some(ref p) = self.path {
            load_wallust_from_path(p)
        } else {
            load_wallust_default()
        }
    }
}

/// Provider for custom theme files on disk.
#[derive(Debug, Clone)]
pub struct FileThemeProvider {
    pub path: PathBuf,
}

impl ThemeProvider for FileThemeProvider {
    fn load_palette(&self) -> Result<ColorPalette, String> {
        load_custom_theme_file(&self.path)
    }
}

/// Provider that automatically queries desktop color caches.
#[derive(Debug, Clone, Default)]
pub struct AutoThemeProvider;

impl ThemeProvider for AutoThemeProvider {
    fn load_palette(&self) -> Result<ColorPalette, String> {
        Ok(detect_auto_theme())
    }
}

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
        return AutoThemeProvider.load_palette();
    }

    if clean.eq_ignore_ascii_case("pywal") {
        return match PywalThemeProvider::default().load_palette() {
            Ok(p) => Ok(p),
            Err(e) => {
                eprintln!("Warning: {e}. Falling back to default lava theme.");
                Ok(get_preset_palette("lava").unwrap_or_default())
            }
        };
    }

    if clean.eq_ignore_ascii_case("wallust") {
        return match WallustThemeProvider::default().load_palette() {
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
        return FileThemeProvider {
            path: PathBuf::from(clean),
        }
        .load_palette();
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
