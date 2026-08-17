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
    // 1. Control modifier checks
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return match key.code {
            KeyCode::Char('c') | KeyCode::Char('C') => Action::Quit,
            _ => Action::None,
        };
    }

    // 2. Alt modifier checks
    if key.modifiers.contains(KeyModifiers::ALT) {
        return Action::None;
    }

    // 3. Command keys and non-command ripple triggers
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Action::Quit,
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

        let plus_event = KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE);
        assert_eq!(map_key_event(plus_event), Action::SpeedUp);

        let minus_event = KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE);
        assert_eq!(map_key_event(minus_event), Action::SlowDown);
    }

    #[test]
    fn test_keyboard_ripple_action() {
        let a_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(map_key_event(a_event), Action::Ripple(1.0));

        let num_event = KeyEvent::new(KeyCode::Char('7'), KeyModifiers::NONE);
        assert_eq!(map_key_event(num_event), Action::Ripple(1.0));

        let sym_event = KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE);
        assert_eq!(map_key_event(sym_event), Action::Ripple(1.0));

        // When ripple disabled:
        assert_eq!(map_key_event_with_ripple(a_event, false), Action::None);
    }

    #[test]
    fn test_modifier_keys_do_not_produce_ripples() {
        // Ctrl+c quits
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(map_key_event(ctrl_c), Action::Quit);

        // Other Ctrl combinations do NOT trigger ripples
        let ctrl_a = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        assert_eq!(map_key_event(ctrl_a), Action::None);

        let ctrl_z = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL);
        assert_eq!(map_key_event(ctrl_z), Action::None);

        // Alt combinations do NOT trigger ripples
        let alt_a = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::ALT);
        assert_eq!(map_key_event(alt_a), Action::None);

        let alt_1 = KeyEvent::new(KeyCode::Char('1'), KeyModifiers::ALT);
        assert_eq!(map_key_event(alt_1), Action::None);
    }
}
