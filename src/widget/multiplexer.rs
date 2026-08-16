//! Environment adapter detecting terminal multiplexer contexts (tmux, zellij).

/// Active terminal multiplexer or generic terminal environment kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplexerKind {
    /// Running inside a tmux session (TMUX variable present).
    Tmux,
    /// Running inside a Zellij session (ZELLIJ variable present).
    Zellij,
    /// Running in a standard/generic terminal emulator.
    GenericTerminal,
}

impl std::fmt::Display for MultiplexerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tmux => write!(f, "tmux"),
            Self::Zellij => write!(f, "zellij"),
            Self::GenericTerminal => write!(f, "generic"),
        }
    }
}

/// Detects the active multiplexer environment by inspecting environment variables.
pub fn detect_multiplexer() -> MultiplexerKind {
    detect_multiplexer_with(|k| std::env::var(k).ok())
}

/// Detects the active multiplexer environment using a custom environment getter for deterministic testing.
pub fn detect_multiplexer_with<F>(mut get_env: F) -> MultiplexerKind
where
    F: FnMut(&str) -> Option<String>,
{
    if let Some(val) = get_env("TMUX") {
        if !val.trim().is_empty() {
            return MultiplexerKind::Tmux;
        }
    }
    if let Some(val) = get_env("ZELLIJ") {
        if !val.trim().is_empty() {
            return MultiplexerKind::Zellij;
        }
    }
    MultiplexerKind::GenericTerminal
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_detect_tmux() {
        let mut env = HashMap::new();
        env.insert("TMUX", "/tmp/tmux-1000/default,1234,0".to_string());
        let kind = detect_multiplexer_with(|k| env.get(k).cloned());
        assert_eq!(kind, MultiplexerKind::Tmux);
        assert_eq!(kind.to_string(), "tmux");
    }

    #[test]
    fn test_detect_zellij() {
        let mut env = HashMap::new();
        env.insert("ZELLIJ", "0".to_string());
        let kind = detect_multiplexer_with(|k| env.get(k).cloned());
        assert_eq!(kind, MultiplexerKind::Zellij);
        assert_eq!(kind.to_string(), "zellij");
    }

    #[test]
    fn test_detect_generic_terminal() {
        let env: HashMap<&str, String> = HashMap::new();
        let kind = detect_multiplexer_with(|k| env.get(k).cloned());
        assert_eq!(kind, MultiplexerKind::GenericTerminal);
        assert_eq!(kind.to_string(), "generic");
    }

    #[test]
    fn test_empty_env_vars_fallback_to_generic() {
        let mut env = HashMap::new();
        env.insert("TMUX", "   ".to_string());
        env.insert("ZELLIJ", "".to_string());
        let kind = detect_multiplexer_with(|k| env.get(k).cloned());
        assert_eq!(kind, MultiplexerKind::GenericTerminal);
    }
}
