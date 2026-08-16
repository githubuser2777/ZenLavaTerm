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
}

impl Config {
    /// Validates configuration parameters and clamps out-of-range values.
    pub fn validate(&self) -> Result<(), String> {
        if self.simulation.blobs == 0 || self.simulation.blobs > 128 {
            return Err("simulation.blobs must be between 1 and 128".to_string());
        }
        if self.render.fps < 1 || self.render.fps > 240 {
            return Err("render.fps must be between 1 and 240".to_string());
        }
        let valid_renderers = ["halfblock", "block", "braille"];
        if !valid_renderers.contains(&self.render.renderer.as_str()) {
            return Err(format!(
                "Invalid renderer '{}'. Must be one of: halfblock, block, braille",
                self.render.renderer
            ));
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

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            blobs: default_blobs(),
            gravity: default_gravity(),
            buoyancy: default_buoyancy(),
            viscosity: default_viscosity(),
            noise: default_noise(),
            threshold: default_threshold(),
        }
    }
}

/// Rendering options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Renderer backend ("halfblock" or "block").
    #[serde(default = "default_renderer")]
    pub renderer: String,

    /// Target frames per second.
    #[serde(default = "default_fps")]
    pub fps: u32,

    /// Enable smooth gradient color mapping.
    #[serde(default = "default_gradient")]
    pub gradient: bool,

    /// Enable double buffering diff optimization.
    #[serde(default = "default_double_buffering")]
    pub double_buffering: bool,
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
fn default_double_buffering() -> bool {
    true
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            renderer: default_renderer(),
            fps: default_fps(),
            gradient: default_gradient(),
            double_buffering: default_double_buffering(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validity() {
        let config = Config::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.simulation.blobs, 12);
        assert_eq!(config.render.fps, 30);
        assert!(!config.reactive.enabled);
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
        "##;

        let config: Config = toml::from_str(toml_str).expect("Valid TOML");
        assert_eq!(config.simulation.blobs, 16);
        assert_eq!(config.render.renderer, "block");
        assert_eq!(config.render.fps, 60);
        assert_eq!(config.palette.bottom, Rgb::new(255, 0, 0));
        assert!(config.reactive.enabled);
        assert_eq!(config.reactive.poll_interval_ms, 250);
    }

    #[test]
    fn test_braille_and_invalid_renderer_validation() {
        let mut config = Config::default();
        config.render.renderer = "braille".to_string();
        assert!(config.validate().is_ok());

        config.render.renderer = "unsupported_renderer".to_string();
        assert!(config.validate().is_err());
    }
}
