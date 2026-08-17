//! Keyboard event mapping and domain actions.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// High-level domain action triggered by user input.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Acoustic ripple / perturbation injected into the lava.
    Ripple(f32),
    /// No actionable command.
    None,
}

/// Translates a crossterm `KeyEvent` into a domain `Action` with optional keyboard ripple.
pub fn map_key_event_with_ripple(key: KeyEvent, enable_ripple: bool) -> Action {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char(' ') | KeyCode::Char('p') | KeyCode::Char('P') => Action::TogglePause,
        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up | KeyCode::Right => Action::SpeedUp,
        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Down | KeyCode::Left => Action::SlowDown,
        KeyCode::Char('r') | KeyCode::Char('R') => Action::Reset,
        KeyCode::Char(_) if enable_ripple => Action::Ripple(1.0),
        _ => Action::None,
    }
}

/// Translates a crossterm `KeyEvent` into a domain `Action` (with keyboard ripple enabled).
pub fn map_key_event(key: KeyEvent) -> Action {
    map_key_event_with_ripple(key, true)
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

        let r_event = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE);
        assert_eq!(map_key_event(r_event), Action::Reset);
    }

    #[test]
    fn test_keyboard_ripple_action() {
        let a_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(map_key_event(a_event), Action::Ripple(1.0));

        let num_event = KeyEvent::new(KeyCode::Char('7'), KeyModifiers::NONE);
        assert_eq!(map_key_event(num_event), Action::Ripple(1.0));

        // When ripple disabled:
        assert_eq!(map_key_event_with_ripple(a_event, false), Action::None);
    }
}
