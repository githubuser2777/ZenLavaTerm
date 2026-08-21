//! Configuration schema migration engine for backward-compatibility with pre-v1.0 formats.

use super::schema::Config;
use crate::{LavaError, Result};
use toml::Value;

/// Migrates legacy or pre-v1.0 TOML configuration structures into current canonical schema.
///
/// Returns `(migrated_config, was_migrated)`.
pub fn migrate_config(raw_toml: &str) -> Result<(Config, bool)> {
    let mut value: Value = toml::from_str(raw_toml)
        .map_err(|e| LavaError::Config(format!("Failed to parse TOML for migration check: {e}")))?;

    let mut migrated = false;

    if let Value::Table(ref mut root_table) = value {
        // 1. Simulation section migrations
        if let Some(Value::Table(ref mut sim_table)) = root_table.get_mut("simulation") {
            if let Some(val) = sim_table.remove("num_blobs") {
                if !sim_table.contains_key("blobs") {
                    sim_table.insert("blobs".to_string(), val);
                    migrated = true;
                }
            }
            if let Some(val) = sim_table.remove("gravity_constant") {
                if !sim_table.contains_key("gravity") {
                    sim_table.insert("gravity".to_string(), val);
                    migrated = true;
                }
            }
            if let Some(val) = sim_table.remove("buoyancy_force") {
                if !sim_table.contains_key("buoyancy") {
                    sim_table.insert("buoyancy".to_string(), val);
                    migrated = true;
                }
            }
        }

        // 2. Render section migrations
        if let Some(Value::Table(ref mut render_table)) = root_table.get_mut("render") {
            if let Some(val) = render_table.remove("renderer_type") {
                if !render_table.contains_key("renderer") {
                    render_table.insert("renderer".to_string(), val);
                    migrated = true;
                }
            }
            if let Some(val) = render_table.remove("target_fps") {
                if !render_table.contains_key("fps") {
                    render_table.insert("fps".to_string(), val);
                    migrated = true;
                }
            }
            if let Some(val) = render_table.remove("smooth_gradient") {
                if !render_table.contains_key("gradient") {
                    render_table.insert("gradient".to_string(), val);
                    migrated = true;
                }
            }
        }

        // 3. Audio section migrations
        if let Some(Value::Table(ref mut audio_table)) = root_table.get_mut("audio") {
            if let Some(val) = audio_table.remove("tempo") {
                if !audio_table.contains_key("bpm") {
                    audio_table.insert("bpm".to_string(), val);
                    migrated = true;
                }
            }
        }

        // 4. Widget section migrations
        if let Some(Value::Table(ref mut widget_table)) = root_table.get_mut("widget") {
            if let Some(val) = widget_table.remove("compact_mode") {
                if !widget_table.contains_key("compact") {
                    widget_table.insert("compact".to_string(), val);
                    migrated = true;
                }
            }
        }
    }

    let serialized = toml::to_string(&value)
        .map_err(|e| LavaError::Config(format!("Failed to serialize migrated TOML: {e}")))?;

    let config: Config = toml::from_str(&serialized).map_err(|e| {
        LavaError::Config(format!("Failed to construct Config after migration: {e}"))
    })?;

    config.validate().map_err(LavaError::Config)?;
    Ok((config, migrated))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_legacy_simulation_and_render_fields() {
        let legacy_toml = r#"
            [simulation]
            num_blobs = 18
            gravity_constant = 0.25

            [render]
            renderer_type = "block"
            target_fps = 45
            smooth_gradient = false

            [audio]
            tempo = 140.0

            [widget]
            compact_mode = true
        "#;

        let (cfg, migrated) = migrate_config(legacy_toml).expect("Migration succeeds");
        assert!(migrated);
        assert_eq!(cfg.simulation.blobs, 18);
        assert!((cfg.simulation.gravity - 0.25).abs() < 1e-4);
        assert_eq!(cfg.render.renderer, "block");
        assert_eq!(cfg.render.fps, 45);
        assert!(!cfg.render.gradient);
        assert_eq!(cfg.audio.bpm, 140.0);
        assert!(cfg.widget.compact);
    }

    #[test]
    fn test_migrate_canonical_config_unmodified() {
        let canonical_toml = r#"
            [simulation]
            blobs = 12
            gravity = 0.12

            [render]
            renderer = "halfblock"
            fps = 30
        "#;

        let (cfg, migrated) = migrate_config(canonical_toml).expect("Canonical config succeeds");
        assert!(!migrated);
        assert_eq!(cfg.simulation.blobs, 12);
        assert_eq!(cfg.render.fps, 30);
    }
}
