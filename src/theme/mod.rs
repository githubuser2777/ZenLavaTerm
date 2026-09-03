//! Theme Engine for LavaTerm.
//!
//! Provides support for built-in curated presets, dynamic Linux desktop color extractors
//! (pywal, wallust), custom theme files (JSON/TOML), and automatic desktop theme detection.

pub mod detector;
pub mod file;
pub mod preset;
pub mod provider;
pub mod pywal;
pub mod wallust;

pub use detector::detect_auto_theme;
pub use file::load_custom_theme_file;
pub use preset::{get_preset_palette, list_presets, ThemePreset, PRESETS};
pub use provider::resolve_theme;
pub use pywal::{load_pywal_default, load_pywal_from_path, parse_pywal_flat, parse_pywal_json};
pub use wallust::{load_wallust_default, load_wallust_from_path, parse_wallust_json};
