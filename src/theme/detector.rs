//! Auto-detection engine for active desktop and terminal color schemes.

use super::{preset::get_preset_palette, pywal::load_pywal_default, wallust::load_wallust_default};
use crate::render::ColorPalette;

/// Theme source information returned alongside detected palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectedThemeSource {
    Pywal,
    Wallust,
    FallbackPreset(&'static str),
}

/// Automatically detects the active system theme from Pywal, Wallust, or default fallback.
pub fn detect_auto_theme() -> (ColorPalette, DetectedThemeSource) {
    // ponytail: linear filesystem probes on startup; inotify/cached watcher if live hot-reloading needed
    if let Ok(pal) = load_pywal_default() {
        return (pal, DetectedThemeSource::Pywal);
    }
    if let Ok(pal) = load_wallust_default() {
        return (pal, DetectedThemeSource::Wallust);
    }
    (
        get_preset_palette("lava").unwrap_or_default(),
        DetectedThemeSource::FallbackPreset("lava"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detection_never_fails() {
        let (pal, source) = detect_auto_theme();
        assert_ne!(pal.bottom, pal.background);
        match source {
            DetectedThemeSource::Pywal
            | DetectedThemeSource::Wallust
            | DetectedThemeSource::FallbackPreset(_) => {}
        }
    }
}
