//! Auto-detection engine for active desktop and terminal color schemes.

use super::{preset::get_preset_palette, pywal::load_pywal_default, wallust::load_wallust_default};
use crate::render::ColorPalette;

/// Automatically detects the active system theme from Pywal, Wallust, or default fallback.
pub fn detect_auto_theme() -> ColorPalette {
    load_pywal_default()
        .or_else(|_| load_wallust_default())
        .unwrap_or_else(|_| get_preset_palette("lava").unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detection_never_fails() {
        let pal = detect_auto_theme();
        assert_ne!(pal.bottom, pal.background);
    }
}
