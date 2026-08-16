//! Keyboard event mapping and domain actions.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// High-level domain action triggered by user input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Exit the application cleanly.
    Quit,
    /// Pause or resume simulation.
    TogglePause,
    /// Increase simulation speed / timestep.
    SpeedUp,
    /// Decrease simulation speed / timestep.
    SlowDown,
    /// Reset simulation state.
    Reset,
    /// No actionable command.
    None,
}

/// Translates a crossterm `KeyEvent` into a domain `Action`.
pub fn map_key_event(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char(' ') | KeyCode::Char('p') | KeyCode::Char('P') => Action::TogglePause,
        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up | KeyCode::Right => Action::SpeedUp,
        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Down | KeyCode::Left => Action::SlowDown,
        KeyCode::Char('r') | KeyCode::Char('R') => Action::Reset,
        _ => Action::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_mapping() {
        let q_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert_eq!(map_key_event(q_event), Action::Quit);

        let esc_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(map_key_event(esc_event), Action::Quit);

        let space_event = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE);
        assert_eq!(map_key_event(space_event), Action::TogglePause);
    }
}
