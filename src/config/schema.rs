//! TOML configuration data structures and validation logic.

use crate::render::{ColorPalette, Rgb};
use serde::{Deserialize, Serialize};

/// Root configuration for LavaTerm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub simulation: SimulationConfig,

    #[serde(default)]
    pub render: RenderConfig,

    #[serde(default)]
    pub palette: PaletteConfig,

    #[serde(default)]
    pub reactive: ReactiveConfig,

    #[serde(default)]
    pub audio: AudioConfig,

    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub widget: WidgetConfig,

    #[serde(default)]
    pub interaction: InteractionConfig,
}

impl Config {
    /// Validates configuration parameters and clamps out-of-range values.
    pub fn validate(&self) -> Result<(), String> {
        if self.simulation.blobs == 0 || self.simulation.blobs > 128 {
            return Err("simulation.blobs must be between 1 and 128".to_string());
        }
        if self.simulation.thermal_transfer_rate <= 0.0
            || self.simulation.thermal_transfer_rate > 5.0
        {
            return Err("simulation.thermal_transfer_rate must be between 0.0 and 5.0".to_string());
        }
        if self.render.fps < 1 || self.render.fps > 240 {
            return Err("render.fps must be between 1 and 240".to_string());
        }
        if self.widget.fps < 1 || self.widget.fps > 240 {
            return Err("widget.fps must be between 1 and 240".to_string());
        }
        if let Some(w) = self.widget.width {
            if w == 0 {
                return Err("widget.width must be greater than 0".to_string());
            }
        }
        if let Some(h) = self.widget.height {
            if h == 0 {
                return Err("widget.height must be greater than 0".to_string());
            }
        }
        let valid_renderers = ["halfblock", "block", "braille"];
        if !valid_renderers.contains(&self.render.renderer.as_str()) {
            return Err(format!(
                "Invalid renderer '{}'. Must be one of: halfblock, block, braille",
                self.render.renderer
            ));
        }
        if self.interaction.shockwave_force <= 0.0 || self.interaction.shockwave_force > 10.0 {
            return Err("interaction.shockwave_force must be between 0.1 and 10.0".to_string());
        }
        if self.interaction.stir_force <= 0.0 || self.interaction.stir_force > 10.0 {
            return Err("interaction.stir_force must be between 0.1 and 10.0".to_string());
        }
        Ok(())
    }
}

/// Simulation-specific parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Number of metaball blobs.
    #[serde(default = "default_blobs")]
    pub blobs: usize,

    /// Gravitational acceleration constant.
    #[serde(default = "default_gravity")]
    pub gravity: f32,

    /// Buoyancy upward multiplier.
    #[serde(default = "default_buoyancy")]
    pub buoyancy: f32,

    /// Viscosity drag factor.
    #[serde(default = "default_viscosity")]
    pub viscosity: f32,

    /// Brownian thermal noise amplitude.
    #[serde(default = "default_noise")]
    pub noise: f32,

    /// Isosurface threshold for lava fluid.
    #[serde(default = "default_threshold")]
    pub threshold: f32,

    /// Rate of thermal transfer with chamber boundaries.
    #[serde(default = "default_thermal_transfer_rate")]
    pub thermal_transfer_rate: f32,
}

fn default_blobs() -> usize {
    12
}
fn default_gravity() -> f32 {
    0.12
}
fn default_buoyancy() -> f32 {
    0.80
}
fn default_viscosity() -> f32 {
    0.93
}
fn default_noise() -> f32 {
    0.15
}
fn default_threshold() -> f32 {
    1.00
}
fn default_thermal_transfer_rate() -> f32 {
    0.40
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            blobs: default_blobs(),
            gravity: default_gravity(),
            buoyancy: default_buoyancy(),
            viscosity: default_viscosity(),
            noise: default_noise(),
            threshold: default_threshold(),
            thermal_transfer_rate: default_thermal_transfer_rate(),
        }
    }
}

/// Rendering options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Renderer backend ("halfblock", "block", or "braille").
    #[serde(default = "default_renderer")]
    pub renderer: String,

    /// Target frames per second.
    #[serde(default = "default_fps")]
    pub fps: u32,

    /// Enable smooth gradient color mapping.
    #[serde(default = "default_gradient")]
    pub gradient: bool,
}

fn default_renderer() -> String {
    "halfblock".to_string()
}
fn default_fps() -> u32 {
    30
}
fn default_gradient() -> bool {
    true
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            renderer: default_renderer(),
            fps: default_fps(),
            gradient: default_gradient(),
        }
    }
}

/// Color palette configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaletteConfig {
    #[serde(default = "default_bottom")]
    pub bottom: Rgb,

    #[serde(default = "default_middle")]
    pub middle: Rgb,

    #[serde(default = "default_top")]
    pub top: Rgb,

    #[serde(default = "default_background")]
    pub background: Rgb,
}

fn default_bottom() -> Rgb {
    Rgb::new(0xFF, 0x3B, 0x00)
}
fn default_middle() -> Rgb {
    Rgb::new(0xFF, 0x7A, 0x00)
}
fn default_top() -> Rgb {
    Rgb::new(0x7B, 0x2C, 0xFF)
}
fn default_background() -> Rgb {
    Rgb::new(0x0D, 0x0D, 0x15)
}

impl Default for PaletteConfig {
    fn default() -> Self {
        Self {
            bottom: default_bottom(),
            middle: default_middle(),
            top: default_top(),
            background: default_background(),
        }
    }
}

impl From<PaletteConfig> for ColorPalette {
    fn from(p: PaletteConfig) -> Self {
        Self {
            bottom: p.bottom,
            middle: p.middle,
            top: p.top,
            background: p.background,
        }
    }
}

/// Reactive system monitoring configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactiveConfig {
    /// Enable system-reactive ambient visualizer mode.
    #[serde(default = "default_reactive_enabled")]
    pub enabled: bool,

    /// Metric polling interval in milliseconds.
    #[serde(default = "default_poll_interval_ms")]
    pub poll_interval_ms: u64,
}

fn default_reactive_enabled() -> bool {
    false
}
fn default_poll_interval_ms() -> u64 {
    500
}

impl Default for ReactiveConfig {
    fn default() -> Self {
        Self {
            enabled: default_reactive_enabled(),
            poll_interval_ms: default_poll_interval_ms(),
        }
    }
}

/// Audio reactive monitoring configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Enable audio-reactive ambient visualizer mode.
    #[serde(default = "default_audio_enabled")]
    pub enabled: bool,

    /// BPM tempo for synthetic fallback beat generator.
    #[serde(default = "default_bpm")]
    pub bpm: f32,
}

fn default_audio_enabled() -> bool {
    false
}
fn default_bpm() -> f32 {
    120.0
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            enabled: default_audio_enabled(),
            bpm: default_bpm(),
        }
    }
}

/// Theme and visual styling configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    /// Active theme name, preset (e.g. "ocean", "cyberpunk", "nord", "synthwave"), "auto", "pywal", or "wallust".
    #[serde(default)]
    pub name: Option<String>,

    /// Optional explicit path to custom theme file (JSON/TOML/wal cache).
    #[serde(default)]
    pub path: Option<std::path::PathBuf>,
}

/// Widget and compact multiplexer configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// Enable compact layout scaling by default.
    #[serde(default)]
    pub compact: bool,

    /// Target frame rate in widget mode.
    #[serde(default = "default_widget_fps")]
    pub fps: u32,

    /// Run in inline mode without alternate screen by default.
    #[serde(default)]
    pub inline: bool,

    /// Optional fixed width for widget layout.
    #[serde(default)]
    pub width: Option<u16>,

    /// Optional fixed height for widget layout.
    #[serde(default)]
    pub height: Option<u16>,

    /// Automatically adapt blob count and physics in compact mode.
    #[serde(default = "default_adapt_blobs")]
    pub adapt_blobs: bool,
}

fn default_widget_fps() -> u32 {
    15
}
fn default_adapt_blobs() -> bool {
    true
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            compact: false,
            fps: default_widget_fps(),
            inline: false,
            width: None,
            height: None,
            adapt_blobs: default_adapt_blobs(),
        }
    }
}

/// Interactive physics and user input configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionConfig {
    /// Enable mouse click shockwaves, dragging, and scroll pressure.
    #[serde(default = "default_interaction_mouse")]
    pub mouse: bool,

    /// Enable keyboard ripples when typing alphanumeric characters.
    #[serde(default = "default_interaction_keyboard_ripple")]
    pub keyboard_ripple: bool,

    /// Multiplier for mouse click shockwave force.
    #[serde(default = "default_shockwave_force")]
    pub shockwave_force: f32,

    /// Multiplier for mouse drag stirring force.
    #[serde(default = "default_stir_force")]
    pub stir_force: f32,
}

fn default_interaction_mouse() -> bool {
    true
}
fn default_interaction_keyboard_ripple() -> bool {
    true
}
fn default_shockwave_force() -> f32 {
    1.0
}
fn default_stir_force() -> f32 {
    1.0
}

impl Default for InteractionConfig {
    fn default() -> Self {
        Self {
            mouse: default_interaction_mouse(),
            keyboard_ripple: default_interaction_keyboard_ripple(),
            shockwave_force: default_shockwave_force(),
            stir_force: default_stir_force(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_config_validation_and_parsing() {
        let toml_str = r##"
            [interaction]
            mouse = false
            keyboard_ripple = true
            shockwave_force = 2.5
            stir_force = 1.8
        "##;

        let config: Config = toml::from_str(toml_str).expect("Valid TOML with interaction");
        assert!(config.validate().is_ok());
        assert!(!config.interaction.mouse);
        assert!(config.interaction.keyboard_ripple);
        assert!((config.interaction.shockwave_force - 2.5).abs() < 1e-4);
        assert!((config.interaction.stir_force - 1.8).abs() < 1e-4);

        let mut invalid = config;
        invalid.interaction.shockwave_force = 0.0;
        assert!(invalid.validate().is_err());

        invalid.interaction.shockwave_force = 1.0;
        invalid.interaction.stir_force = -0.5;
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_default_config_validity() {
        let config = Config::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.simulation.blobs, 12);
        assert_eq!(config.render.fps, 30);
        assert!(!config.reactive.enabled);
        assert!(!config.audio.enabled);
        assert_eq!(config.theme.name, None);
    }

    #[test]
    fn test_toml_parsing() {
        let toml_str = r##"
            [simulation]
            blobs = 16
            gravity = 0.2

            [render]
            renderer = "block"
            fps = 60

            [palette]
            bottom = "#ff0000"
            middle = "#ffff00"
            top = "#0000ff"
            background = "#000000"

            [reactive]
            enabled = true
            poll_interval_ms = 250

            [audio]
            enabled = true
            bpm = 128.0

            [theme]
            name = "cyberpunk"
        "##;

        let config: Config = toml::from_str(toml_str).expect("Valid TOML");
        assert_eq!(config.simulation.blobs, 16);
        assert_eq!(config.render.renderer, "block");
        assert_eq!(config.render.fps, 60);
        assert_eq!(config.palette.bottom, Rgb::new(255, 0, 0));
        assert!(config.reactive.enabled);
        assert_eq!(config.reactive.poll_interval_ms, 250);
        assert!(config.audio.enabled);
        assert_eq!(config.audio.bpm, 128.0);
        assert_eq!(config.theme.name.as_deref(), Some("cyberpunk"));
        assert!(!config.widget.compact);
        assert_eq!(config.widget.fps, 15);
    }

    #[test]
    fn test_widget_toml_parsing_and_validation() {
        let toml_str = r##"
            [widget]
            compact = true
            fps = 20
            inline = true
            width = 30
            height = 10
            adapt_blobs = false
        "##;

        let config: Config = toml::from_str(toml_str).expect("Valid TOML with widget");
        assert!(config.validate().is_ok());
        assert!(config.widget.compact);
        assert_eq!(config.widget.fps, 20);
        assert!(config.widget.inline);
        assert_eq!(config.widget.width, Some(30));
        assert_eq!(config.widget.height, Some(10));
        assert!(!config.widget.adapt_blobs);

        let mut invalid = config;
        invalid.widget.fps = 0;
        assert!(invalid.validate().is_err());

        invalid.widget.fps = 15;
        invalid.widget.width = Some(0);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_braille_and_invalid_renderer_validation() {
        let mut config = Config::default();
        config.render.renderer = "braille".to_string();
        assert!(config.validate().is_ok());

        config.render.renderer = "unsupported_renderer".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_thermal_transfer_rate_validation() {
        let mut config = Config::default();
        assert_eq!(config.simulation.thermal_transfer_rate, 0.40);
        assert!(config.validate().is_ok());

        config.simulation.thermal_transfer_rate = 0.0;
        assert!(config.validate().is_err());

        config.simulation.thermal_transfer_rate = -0.5;
        assert!(config.validate().is_err());

        config.simulation.thermal_transfer_rate = 10.0;
        assert!(config.validate().is_err());

        config.simulation.thermal_transfer_rate = 1.5;
        assert!(config.validate().is_ok());
    }
}
