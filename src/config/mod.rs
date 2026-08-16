//! Configuration loading, file discovery, and defaults.

pub mod schema;

pub use schema::{Config, PaletteConfig, RenderConfig, SimulationConfig};

use crate::{LavaError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Loads a configuration from a specific file path.
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref).map_err(|e| {
        LavaError::Config(format!(
            "Failed to read config file at '{}': {}",
            path_ref.display(),
            e
        ))
    })?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        LavaError::Config(format!(
            "Failed to parse TOML config at '{}': {}",
            path_ref.display(),
            e
        ))
    })?;

    config.validate().map_err(LavaError::Config)?;
    Ok(config)
}

/// Discovers standard configuration paths (`~/.config/lavaterm/config.toml`).
pub fn default_config_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home)
            .join(".config")
            .join("lavaterm")
            .join("config.toml");
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// Loads configuration from custom path, auto-discovered path, or defaults.
pub fn load_config(custom_path: Option<&Path>) -> Result<Config> {
    if let Some(p) = custom_path {
        return load_from_path(p);
    }

    if let Some(p) = default_config_path() {
        return load_from_path(p);
    }

    Ok(Config::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_when_no_path() {
        let cfg = load_config(None).expect("Default config succeeds");
        assert_eq!(cfg.simulation.blobs, 12);
    }
}
