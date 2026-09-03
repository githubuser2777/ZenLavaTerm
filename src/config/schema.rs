//! TOML configuration data structures and validation logic.

use crate::render::ColorPalette;
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
        if !self.simulation.gravity.is_finite()
            || self.simulation.gravity < 0.0
            || self.simulation.gravity > 5.0
        {
            return Err(
                "simulation.gravity must be a finite number between 0.0 and 5.0".to_string(),
            );
        }
        if !self.simulation.buoyancy.is_finite()
            || self.simulation.buoyancy < 0.0
            || self.simulation.buoyancy > 5.0
        {
            return Err(
                "simulation.buoyancy must be a finite number between 0.0 and 5.0".to_string(),
            );
        }
        if !self.simulation.viscosity.is_finite()
            || self.simulation.viscosity < 0.0
            || self.simulation.viscosity > 1.0
        {
            return Err(
                "simulation.viscosity must be a finite number between 0.0 and 1.0".to_string(),
            );
        }
        if !self.simulation.noise.is_finite()
            || self.simulation.noise < 0.0
            || self.simulation.noise > 2.0
        {
            return Err("simulation.noise must be a finite number between 0.0 and 2.0".to_string());
        }
        if !self.simulation.threshold.is_finite()
            || self.simulation.threshold < 0.1
            || self.simulation.threshold > 10.0
        {
            return Err(
                "simulation.threshold must be a finite number between 0.1 and 10.0".to_string(),
            );
        }
        if !self.simulation.thermal_transfer_rate.is_finite()
            || self.simulation.thermal_transfer_rate <= 0.0
            || self.simulation.thermal_transfer_rate > 5.0
        {
            return Err(
                "simulation.thermal_transfer_rate must be a finite number between > 0.0 and 5.0"
                    .to_string(),
            );
        }
        if !self.audio.bpm.is_finite() || self.audio.bpm < 20.0 || self.audio.bpm > 300.0 {
            return Err("audio.bpm must be a finite number between 20.0 and 300.0".to_string());
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
        if !self.interaction.shockwave_force.is_finite()
            || self.interaction.shockwave_force < 0.1
            || self.interaction.shockwave_force > 10.0
        {
            return Err(
                "interaction.shockwave_force must be a finite number between 0.1 and 10.0"
                    .to_string(),
            );
        }
        if !self.interaction.stir_force.is_finite()
            || self.interaction.stir_force < 0.1
            || self.interaction.stir_force > 10.0
        {
            return Err(
                "interaction.stir_force must be a finite number between 0.1 and 10.0".to_string(),
            );
        }
        Ok(())
    }
}

/// Simulation-specific parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SimulationConfig {
    /// Number of metaball blobs.
    #[serde(alias = "num_blobs")]
    pub blobs: usize,

    /// Gravitational acceleration constant.
    #[serde(alias = "gravity_constant")]
    pub gravity: f32,

    /// Buoyancy upward multiplier.
    #[serde(alias = "buoyancy_force")]
    pub buoyancy: f32,

    /// Viscosity drag factor.
    pub viscosity: f32,

    /// Brownian thermal noise amplitude.
    pub noise: f32,

    /// Isosurface threshold for lava fluid.
    pub threshold: f32,

    /// Rate of thermal transfer with chamber boundaries.
    pub thermal_transfer_rate: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            blobs: 12,
            gravity: 0.12,
            buoyancy: 0.80,
            viscosity: 0.93,
            noise: 0.15,
            threshold: 1.00,
            thermal_transfer_rate: 0.40,
        }
    }
}

/// Rendering options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderConfig {
    /// Renderer backend ("halfblock", "block", or "braille").
    #[serde(alias = "renderer_type")]
    pub renderer: String,

    /// Target frames per second.
    #[serde(alias = "target_fps")]
    pub fps: u32,

    /// Enable smooth gradient color mapping.
    #[serde(alias = "smooth_gradient")]
    pub gradient: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            renderer: "halfblock".to_string(),
            fps: 30,
            gradient: true,
        }
    }
}

/// Color palette configuration (re-exports `ColorPalette` directly).
pub type PaletteConfig = ColorPalette;

/// Reactive system monitoring configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReactiveConfig {
    /// Enable system-reactive ambient visualizer mode.
    pub enabled: bool,

    /// Metric polling interval in milliseconds.
    pub poll_interval_ms: u64,
}

impl Default for ReactiveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            poll_interval_ms: 500,
        }
    }
}

/// Audio reactive monitoring configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioConfig {
    /// Enable audio-reactive ambient visualizer mode.
    pub enabled: bool,

    /// BPM tempo for synthetic fallback beat generator.
    #[serde(alias = "tempo")]
    pub bpm: f32,

    /// Optional target audio input/capture device name.
    #[serde(default)]
    pub device: Option<String>,
    /// Whether to capture system output audio (loopback) instead of microphone input.
    #[serde(default)]
    pub loopback: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bpm: 120.0,
            device: None,
            loopback: false,
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
#[serde(default)]
pub struct WidgetConfig {
    /// Enable compact layout scaling by default.
    #[serde(alias = "compact_mode")]
    pub compact: bool,

    /// Target frame rate in widget mode.
    pub fps: u32,

    /// Run in inline mode without alternate screen by default.
    pub inline: bool,

    /// Optional fixed width for widget layout.
    pub width: Option<u16>,

    /// Optional fixed height for widget layout.
    pub height: Option<u16>,

    /// Automatically adapt blob count and physics in compact mode.
    pub adapt_blobs: bool,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            compact: false,
            fps: 15,
            inline: false,
            width: None,
            height: None,
            adapt_blobs: true,
        }
    }
}

/// Interactive physics and user input configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InteractionConfig {
    /// Enable mouse click shockwaves, dragging, and scroll pressure.
    pub mouse: bool,

    /// Enable keyboard ripples when typing alphanumeric characters.
    pub keyboard_ripple: bool,

    /// Multiplier for mouse click shockwave force.
    pub shockwave_force: f32,

    /// Multiplier for mouse drag stirring force.
    pub stir_force: f32,
}

impl Default for InteractionConfig {
    fn default() -> Self {
        Self {
            mouse: true,
            keyboard_ripple: true,
            shockwave_force: 1.0,
            stir_force: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Rgb;

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

        invalid.interaction.shockwave_force = 0.09;
        assert!(invalid.validate().is_err());

        invalid.interaction.shockwave_force = 10.01;
        assert!(invalid.validate().is_err());

        invalid.interaction.shockwave_force = 1.0;
        invalid.interaction.stir_force = -0.5;
        assert!(invalid.validate().is_err());

        invalid.interaction.stir_force = 0.05;
        assert!(invalid.validate().is_err());

        invalid.interaction.stir_force = 10.5;
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_force_boundary_values() {
        let mut config = Config::default();

        // Exact lower bound 0.1
        config.interaction.shockwave_force = 0.1;
        config.interaction.stir_force = 0.1;
        assert!(config.validate().is_ok());

        // Exact upper bound 10.0
        config.interaction.shockwave_force = 10.0;
        config.interaction.stir_force = 10.0;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_nan_and_infinity_rejection_across_all_fields() {
        let mut config = Config::default();

        // Test interaction forces with NaN and Infinity
        config.interaction.shockwave_force = f32::NAN;
        assert!(config.validate().is_err());
        config.interaction.shockwave_force = f32::INFINITY;
        assert!(config.validate().is_err());
        config.interaction.shockwave_force = f32::NEG_INFINITY;
        assert!(config.validate().is_err());
        config.interaction.shockwave_force = 1.0;

        config.interaction.stir_force = f32::NAN;
        assert!(config.validate().is_err());
        config.interaction.stir_force = f32::INFINITY;
        assert!(config.validate().is_err());
        config.interaction.stir_force = f32::NEG_INFINITY;
        assert!(config.validate().is_err());
        config.interaction.stir_force = 1.0;

        // Test simulation floats with NaN and Infinity
        config.simulation.gravity = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.gravity = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.gravity = 0.12;

        config.simulation.buoyancy = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.buoyancy = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.buoyancy = 0.80;

        config.simulation.viscosity = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.viscosity = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.viscosity = 0.93;

        config.simulation.noise = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.noise = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.noise = 0.15;

        config.simulation.threshold = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.threshold = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.threshold = 1.00;

        config.simulation.thermal_transfer_rate = f32::NAN;
        assert!(config.validate().is_err());
        config.simulation.thermal_transfer_rate = f32::INFINITY;
        assert!(config.validate().is_err());
        config.simulation.thermal_transfer_rate = 0.40;

        // Test audio BPM with NaN and Infinity
        config.audio.bpm = f32::NAN;
        assert!(config.validate().is_err());
        config.audio.bpm = f32::INFINITY;
        assert!(config.validate().is_err());
        config.audio.bpm = 120.0;

        // Ensure cleanly restored
        assert!(config.validate().is_ok());
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
