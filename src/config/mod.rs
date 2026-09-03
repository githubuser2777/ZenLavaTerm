//! Configuration loading, file discovery, and defaults.

pub mod schema;

pub use schema::{AudioConfig, Config, PaletteConfig, RenderConfig, SimulationConfig};

use crate::{LavaError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Loads a configuration from a specific file path, supporting backward-compatible schema aliases via Serde.
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref).map_err(|e| {
        LavaError::Config(format!(
            "Failed to read config file at '{}': {}",
            path_ref.display(),
            e
        ))
    })?;

    let config: Config = toml::from_str(&content)
        .map_err(|e| LavaError::Config(format!("Failed to parse config TOML: {e}")))?;
    config.validate().map_err(LavaError::Config)?;
    Ok(config)
}

/// Discovers standard configuration paths according to host platform conventions:
/// - Linux: `$XDG_CONFIG_HOME/lavaterm/config.toml` -> `$HOME/.config/lavaterm/config.toml`
/// - macOS: `$XDG_CONFIG_HOME` (if set) -> `$HOME/Library/Application Support/lavaterm/config.toml` -> `$HOME/.config/lavaterm/config.toml`
/// - Windows: `%APPDATA%\lavaterm\config.toml` -> `%USERPROFILE%\AppData\Roaming\lavaterm\config.toml` -> `%USERPROFILE%\.config` -> `$XDG_CONFIG_HOME`
pub fn default_config_path() -> Option<PathBuf> {
    default_config_path_with(|k| std::env::var(k).ok())
}

/// Discovers standard configuration paths using a custom environment getter on the current OS.
pub fn default_config_path_with<F>(get_env: F) -> Option<PathBuf>
where
    F: Fn(&str) -> Option<String>,
{
    default_config_path_for_os(std::env::consts::OS, get_env)
}

/// Discovers standard configuration paths according to a specific target operating system convention.
pub fn default_config_path_for_os<F>(os: &str, get_env: F) -> Option<PathBuf>
where
    F: Fn(&str) -> Option<String>,
{
    match os {
        "macos" => {
            // 1. Explicit XDG_CONFIG_HOME
            if let Some(xdg) = get_env("XDG_CONFIG_HOME") {
                let trimmed = xdg.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed).join("lavaterm").join("config.toml");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }

            // 2. Native macOS Application Support -> Fallback ~/.config
            if let Some(home) = get_env("HOME") {
                let trimmed = home.trim();
                if !trimmed.is_empty() {
                    let p_appsupport = PathBuf::from(trimmed)
                        .join("Library")
                        .join("Application Support")
                        .join("lavaterm")
                        .join("config.toml");
                    if p_appsupport.exists() {
                        return Some(p_appsupport);
                    }

                    let p_dotconfig = PathBuf::from(trimmed)
                        .join(".config")
                        .join("lavaterm")
                        .join("config.toml");
                    if p_dotconfig.exists() {
                        return Some(p_dotconfig);
                    }
                }
            }
        }
        "windows" => {
            // 1. Standard Windows APPDATA
            if let Some(appdata) = get_env("APPDATA") {
                let trimmed = appdata.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed).join("lavaterm").join("config.toml");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }

            // 2. Windows USERPROFILE fallback
            if let Some(userprofile) = get_env("USERPROFILE") {
                let trimmed = userprofile.trim();
                if !trimmed.is_empty() {
                    let p_roaming = PathBuf::from(trimmed)
                        .join("AppData")
                        .join("Roaming")
                        .join("lavaterm")
                        .join("config.toml");
                    if p_roaming.exists() {
                        return Some(p_roaming);
                    }

                    let p_dotconfig = PathBuf::from(trimmed)
                        .join(".config")
                        .join("lavaterm")
                        .join("config.toml");
                    if p_dotconfig.exists() {
                        return Some(p_dotconfig);
                    }
                }
            }

            // 3. Optional XDG fallback on Windows
            if let Some(xdg) = get_env("XDG_CONFIG_HOME") {
                let trimmed = xdg.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed).join("lavaterm").join("config.toml");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }
        _ => {
            // Linux and other Unix-like OS precedence
            if let Some(xdg) = get_env("XDG_CONFIG_HOME") {
                let trimmed = xdg.trim();
                if !trimmed.is_empty() {
                    let p = PathBuf::from(trimmed).join("lavaterm").join("config.toml");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }

            if let Some(home) = get_env("HOME") {
                let trimmed = home.trim();
                if !trimmed.is_empty() {
                    let p_dotconfig = PathBuf::from(trimmed)
                        .join(".config")
                        .join("lavaterm")
                        .join("config.toml");
                    if p_dotconfig.exists() {
                        return Some(p_dotconfig);
                    }
                }
            }
        }
    }

    None
}

/// Loads configuration from custom path, auto-discovered path, or defaults.
/// Every configuration returned by this function is guaranteed to pass validation.
pub fn load_config(custom_path: Option<&Path>) -> Result<Config> {
    if let Some(p) = custom_path {
        return load_from_path(p);
    }

    if let Some(p) = default_config_path() {
        return load_from_path(p);
    }

    let config = Config::default();
    config.validate().map_err(LavaError::Config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_load_default_when_no_path() {
        let cfg = load_config(None).expect("Default config succeeds");
        assert_eq!(cfg.simulation.blobs, 12);
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_default_config_path_with_empty_or_missing_env() {
        let env: HashMap<&str, String> = HashMap::new();
        assert_eq!(default_config_path_with(|k| env.get(k).cloned()), None);

        let mut env_empty = HashMap::new();
        env_empty.insert("XDG_CONFIG_HOME", "   ".to_string());
        env_empty.insert("APPDATA", " ".to_string());
        env_empty.insert("HOME", "".to_string());
        env_empty.insert("USERPROFILE", "  ".to_string());
        assert_eq!(
            default_config_path_with(|k| env_empty.get(k).cloned()),
            None
        );
    }

    #[test]
    fn test_default_config_path_linux_precedence() {
        let temp_dir = std::env::temp_dir();
        let test_xdg = temp_dir.join(format!("lavaterm_linux_xdg_{}", std::process::id()));
        let test_home = temp_dir.join(format!("lavaterm_linux_home_{}", std::process::id()));

        let xdg_config_dir = test_xdg.join("lavaterm");
        let home_config_dir = test_home.join(".config").join("lavaterm");

        fs::create_dir_all(&xdg_config_dir).expect("failed to create xdg test dir");
        fs::create_dir_all(&home_config_dir).expect("failed to create home test dir");

        let xdg_file = xdg_config_dir.join("config.toml");
        let home_file = home_config_dir.join("config.toml");

        fs::write(&xdg_file, "# xdg config").expect("failed to write xdg config");
        fs::write(&home_file, "# home config").expect("failed to write home config");

        let mut env = HashMap::new();
        env.insert("XDG_CONFIG_HOME", test_xdg.to_string_lossy().to_string());
        env.insert("HOME", test_home.to_string_lossy().to_string());

        // On Linux: XDG should take precedence over ~/.config
        let found = default_config_path_for_os("linux", |k| env.get(k).cloned());
        assert_eq!(found, Some(xdg_file.clone()));

        // Fallback to ~/.config
        let _ = fs::remove_file(&xdg_file);
        let found_fallback = default_config_path_for_os("linux", |k| env.get(k).cloned());
        assert_eq!(found_fallback, Some(home_file.clone()));

        // Cleanup
        let _ = fs::remove_file(&home_file);
        let _ = fs::remove_dir_all(&test_xdg);
        let _ = fs::remove_dir_all(&test_home);
    }

    #[test]
    fn test_default_config_path_macos_precedence_over_dotconfig() {
        let temp_dir = std::env::temp_dir();
        let test_home = temp_dir.join(format!("lavaterm_macos_home_{}", std::process::id()));

        let app_support_dir = test_home
            .join("Library")
            .join("Application Support")
            .join("lavaterm");
        let dotconfig_dir = test_home.join(".config").join("lavaterm");

        fs::create_dir_all(&app_support_dir).expect("failed to create app support test dir");
        fs::create_dir_all(&dotconfig_dir).expect("failed to create dotconfig test dir");

        let app_support_file = app_support_dir.join("config.toml");
        let dotconfig_file = dotconfig_dir.join("config.toml");

        fs::write(&app_support_file, "# macos native app support config")
            .expect("failed to write app support config");
        fs::write(&dotconfig_file, "# macos unix dotconfig").expect("failed to write dotconfig");

        let mut env = HashMap::new();
        env.insert("HOME", test_home.to_string_lossy().to_string());

        // On macOS: Library/Application Support MUST take precedence over ~/.config
        let found = default_config_path_for_os("macos", |k| env.get(k).cloned());
        assert_eq!(found, Some(app_support_file.clone()));

        // When Application Support file is missing, fallback to ~/.config
        let _ = fs::remove_file(&app_support_file);
        let found_fallback = default_config_path_for_os("macos", |k| env.get(k).cloned());
        assert_eq!(found_fallback, Some(dotconfig_file.clone()));

        // Cleanup
        let _ = fs::remove_file(&dotconfig_file);
        let _ = fs::remove_dir_all(&test_home);
    }

    #[test]
    fn test_default_config_path_windows_precedence() {
        let temp_dir = std::env::temp_dir();
        let test_appdata = temp_dir.join(format!("lavaterm_win_appdata_{}", std::process::id()));
        let test_userprofile =
            temp_dir.join(format!("lavaterm_win_userprofile_{}", std::process::id()));
        let test_xdg = temp_dir.join(format!("lavaterm_win_xdg_{}", std::process::id()));

        let appdata_config_dir = test_appdata.join("lavaterm");
        let userprofile_roaming = test_userprofile
            .join("AppData")
            .join("Roaming")
            .join("lavaterm");
        let xdg_config_dir = test_xdg.join("lavaterm");

        fs::create_dir_all(&appdata_config_dir).expect("failed to create appdata test dir");
        fs::create_dir_all(&userprofile_roaming).expect("failed to create userprofile test dir");
        fs::create_dir_all(&xdg_config_dir).expect("failed to create xdg test dir");

        let appdata_file = appdata_config_dir.join("config.toml");
        let userprofile_file = userprofile_roaming.join("config.toml");
        let xdg_file = xdg_config_dir.join("config.toml");

        fs::write(&appdata_file, "# appdata config").expect("failed to write appdata config");
        fs::write(&userprofile_file, "# userprofile config")
            .expect("failed to write userprofile config");
        fs::write(&xdg_file, "# xdg config").expect("failed to write xdg config");

        let mut env = HashMap::new();
        env.insert("APPDATA", test_appdata.to_string_lossy().to_string());
        env.insert(
            "USERPROFILE",
            test_userprofile.to_string_lossy().to_string(),
        );
        env.insert("XDG_CONFIG_HOME", test_xdg.to_string_lossy().to_string());

        // On Windows: APPDATA MUST take precedence over USERPROFILE and XDG
        let found = default_config_path_for_os("windows", |k| env.get(k).cloned());
        assert_eq!(found, Some(appdata_file.clone()));

        // When APPDATA file is removed, fall back to USERPROFILE
        let _ = fs::remove_file(&appdata_file);
        let found_fallback = default_config_path_for_os("windows", |k| env.get(k).cloned());
        assert_eq!(found_fallback, Some(userprofile_file.clone()));

        // When USERPROFILE file is removed, fall back to XDG
        let _ = fs::remove_file(&userprofile_file);
        let found_xdg_fallback = default_config_path_for_os("windows", |k| env.get(k).cloned());
        assert_eq!(found_xdg_fallback, Some(xdg_file.clone()));

        // Cleanup
        let _ = fs::remove_file(&xdg_file);
        let _ = fs::remove_dir_all(&test_appdata);
        let _ = fs::remove_dir_all(&test_userprofile);
        let _ = fs::remove_dir_all(&test_xdg);
    }
}
