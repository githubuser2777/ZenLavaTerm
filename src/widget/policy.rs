//! Widget execution policy, mode resolution, and conflict validation.

use crate::{LavaError, Result};

/// Execution mode for the LavaTerm application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Standard full-screen interactive loop in alternate screen.
    Interactive,
    /// Low-overhead ambient widget loop with reduced default FPS and compact scaling.
    Widget,
    /// In-place interactive loop without entering the alternate screen.
    Inline,
    /// Single-shot frame serialization directly to stdout for status bars.
    Snapshot,
    /// Headless non-interactive simulation stepping for testing and CI.
    Headless,
}

/// Raw input parameters from CLI flags and TOML configuration.
#[derive(Debug, Clone, Default)]
pub struct PolicyInput {
    pub cli_fps: Option<u32>,
    pub cli_compact: bool,
    pub cli_widget: bool,
    pub cli_inline: bool,
    pub cli_snapshot: bool,
    pub cli_headless: bool,
    pub cli_width: Option<u16>,
    pub cli_height: Option<u16>,

    pub toml_render_fps: u32,
    pub toml_widget_fps: u32,
    pub toml_widget_compact: bool,
    pub toml_widget_inline: bool,
    pub toml_widget_width: Option<u16>,
    pub toml_widget_height: Option<u16>,
    pub toml_widget_adapt_blobs: bool,
}

/// Resolved runtime policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPolicy {
    pub mode: ExecutionMode,
    pub target_fps: u32,
    pub force_compact: bool,
    pub explicit_dimensions: Option<(u16, u16)>,
    pub adapt_blobs: bool,
}

/// Resolves raw inputs into a validated `ResolvedPolicy` following strict precedence rules.
///
/// Precedence: CLI Arguments > TOML Configuration > Built-in Defaults.
pub fn resolve_policy(input: &PolicyInput) -> Result<ResolvedPolicy> {
    // 1. Conflict Validation
    if input.cli_snapshot && (input.cli_inline || input.cli_headless) {
        return Err(LavaError::Config(
            "Conflict: --snapshot cannot be combined with --inline or --headless".to_string(),
        ));
    }
    if input.cli_inline && input.cli_headless {
        return Err(LavaError::Config(
            "Conflict: --inline cannot be combined with --headless".to_string(),
        ));
    }

    // 2. Resolve ExecutionMode
    let mode = if input.cli_snapshot {
        ExecutionMode::Snapshot
    } else if input.cli_headless {
        ExecutionMode::Headless
    } else if input.cli_inline || input.toml_widget_inline {
        ExecutionMode::Inline
    } else if input.cli_widget {
        ExecutionMode::Widget
    } else {
        ExecutionMode::Interactive
    };

    // 3. Resolve Target FPS
    // Precedence: CLI --fps > (Widget mode default 15 / toml_widget_fps) > (Interactive toml_render_fps) > 30
    if let Some(0) = input.cli_fps {
        return Err(LavaError::Config(
            "Target FPS must be greater than zero".to_string(),
        ));
    }
    let target_fps = if let Some(fps) = input.cli_fps {
        fps
    } else if mode == ExecutionMode::Widget {
        if input.toml_widget_fps > 0 {
            input.toml_widget_fps
        } else {
            15
        }
    } else if input.toml_render_fps > 0 {
        input.toml_render_fps
    } else {
        30
    };

    if target_fps == 0 {
        return Err(LavaError::Config(
            "Target FPS must be greater than zero".to_string(),
        ));
    }

    // 4. Resolve Compact Policy
    // --widget implies compact mode; --compact forces it; toml [widget].compact can configure it
    let force_compact = input.cli_compact
        || input.cli_widget
        || input.toml_widget_compact
        || mode == ExecutionMode::Widget;

    // 5. Resolve Explicit Dimensions
    let width = input.cli_width.or(input.toml_widget_width);
    let height = input.cli_height.or(input.toml_widget_height);
    let explicit_dimensions = match (width, height) {
        (Some(w), Some(h)) => {
            if w == 0 || h == 0 {
                return Err(LavaError::Config(
                    "Explicit dimensions (width/height) must be greater than zero".to_string(),
                ));
            }
            Some((w, h))
        }
        (Some(_), None) => {
            return Err(LavaError::Config(
                "Must specify both --width and --height together".to_string(),
            ));
        }
        (None, Some(_)) => {
            return Err(LavaError::Config(
                "Must specify both --width and --height together".to_string(),
            ));
        }
        (None, None) => None,
    };

    let adapt_blobs = input.toml_widget_adapt_blobs;

    Ok(ResolvedPolicy {
        mode,
        target_fps,
        force_compact,
        explicit_dimensions,
        adapt_blobs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_zero_rejected() {
        let input_cli_zero = PolicyInput {
            cli_fps: Some(0),
            ..Default::default()
        };
        let err = resolve_policy(&input_cli_zero).expect_err("CLI FPS=0 must be rejected");
        assert!(err.to_string().contains("greater than zero"));

        let input_all_zero = PolicyInput {
            toml_render_fps: 0,
            toml_widget_fps: 0,
            ..Default::default()
        };
        let policy = resolve_policy(&input_all_zero).expect("Fallback default FPS must be > 0");
        assert_eq!(policy.target_fps, 30);
    }

    #[test]
    fn test_default_policy_resolution() {
        let input = PolicyInput {
            toml_render_fps: 30,
            toml_widget_fps: 15,
            toml_widget_adapt_blobs: true,
            ..Default::default()
        };

        let policy = resolve_policy(&input).expect("Valid default resolution");
        assert_eq!(policy.mode, ExecutionMode::Interactive);
        assert_eq!(policy.target_fps, 30);
        assert!(!policy.force_compact);
        assert_eq!(policy.explicit_dimensions, None);
        assert!(policy.adapt_blobs);
    }

    #[test]
    fn test_widget_mode_defaults_to_15_fps_and_compact() {
        let input = PolicyInput {
            cli_widget: true,
            toml_render_fps: 60,
            toml_widget_fps: 15,
            toml_widget_adapt_blobs: true,
            ..Default::default()
        };

        let policy = resolve_policy(&input).expect("Widget mode resolution");
        assert_eq!(policy.mode, ExecutionMode::Widget);
        assert_eq!(policy.target_fps, 15);
        assert!(policy.force_compact);
    }

    #[test]
    fn test_cli_fps_overrides_all() {
        let input = PolicyInput {
            cli_widget: true,
            cli_fps: Some(25),
            toml_render_fps: 60,
            toml_widget_fps: 15,
            ..Default::default()
        };

        let policy = resolve_policy(&input).expect("CLI fps override resolution");
        assert_eq!(policy.target_fps, 25);
    }

    #[test]
    fn test_snapshot_conflict_validation() {
        let input = PolicyInput {
            cli_snapshot: true,
            cli_inline: true,
            ..Default::default()
        };

        let err = resolve_policy(&input).expect_err("Snapshot and inline must conflict");
        assert!(err.to_string().contains("Conflict"));
    }

    #[test]
    fn test_dimension_validation() {
        let input_partial = PolicyInput {
            cli_width: Some(20),
            cli_height: None,
            ..Default::default()
        };
        assert!(resolve_policy(&input_partial).is_err());

        let input_zero = PolicyInput {
            cli_width: Some(0),
            cli_height: Some(10),
            ..Default::default()
        };
        assert!(resolve_policy(&input_zero).is_err());

        let input_valid = PolicyInput {
            cli_width: Some(20),
            cli_height: Some(5),
            ..Default::default()
        };
        let policy = resolve_policy(&input_valid).expect("Valid dimensions");
        assert_eq!(policy.explicit_dimensions, Some((20, 5)));
    }
}
