//! Mouse event tracking and translation into domain interactions.

use super::coords::terminal_to_sim_coords;
use crate::core::interaction::Interaction;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

/// Stateful mouse tracker that converts mouse movements and gestures into fluid interactions.
#[derive(Debug, Clone, Default)]
pub struct MouseTracker {
    /// Previous continuous position of the mouse during dragging.
    last_drag_pos: Option<(f32, f32)>,
}

impl MouseTracker {
    /// Creates a new empty mouse tracker.
    pub fn new() -> Self {
        Self {
            last_drag_pos: None,
        }
    }

    /// Processes a crossterm `MouseEvent` and returns an optional domain `Interaction`.
    pub fn handle_event(
        &mut self,
        event: MouseEvent,
        cols: u16,
        rows: u16,
        shockwave_force: f32,
        stir_force: f32,
    ) -> Option<Interaction> {
        let (sim_x, sim_y) = terminal_to_sim_coords(event.column, event.row, cols, rows);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.last_drag_pos = Some((sim_x, sim_y));
                Some(Interaction::Shockwave {
                    x: sim_x,
                    y: sim_y,
                    force: shockwave_force,
                })
            }
            MouseEventKind::Down(MouseButton::Right) => {
                self.last_drag_pos = Some((sim_x, sim_y));
                Some(Interaction::ThermalPulse {
                    x: sim_x,
                    y: sim_y,
                    temperature_delta: 0.60,
                    radius: 0.20,
                })
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some((last_x, last_y)) = self.last_drag_pos {
                    let dx = sim_x - last_x;
                    let dy = sim_y - last_y;
                    self.last_drag_pos = Some((sim_x, sim_y));

                    // Scale displacement into velocity impulse
                    let vx = dx * 6.0 * stir_force;
                    let vy = dy * 6.0 * stir_force;

                    Some(Interaction::Stir {
                        x: sim_x,
                        y: sim_y,
                        vx,
                        vy,
                        radius: 0.22,
                    })
                } else {
                    self.last_drag_pos = Some((sim_x, sim_y));
                    None
                }
            }
            MouseEventKind::Up(_) => {
                self.last_drag_pos = None;
                None
            }
            MouseEventKind::ScrollUp => Some(Interaction::Pressure { delta: 1.0 }),
            MouseEventKind::ScrollDown => Some(Interaction::Pressure { delta: -1.0 }),
            _ => None,
        }
    }

    /// Resets the tracker state (e.g. on focus loss).
    pub fn reset(&mut self) {
        self.last_drag_pos = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn test_mouse_left_click_emits_shockwave() {
        let mut tracker = MouseTracker::new();
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 40,
            row: 12,
            modifiers: KeyModifiers::NONE,
        };

        let interaction = tracker.handle_event(event, 80, 24, 1.5, 1.0);
        assert!(matches!(
            interaction,
            Some(Interaction::Shockwave { force, .. }) if (force - 1.5).abs() < 1e-4
        ));
    }

    #[test]
    fn test_mouse_right_click_emits_thermal_pulse() {
        let mut tracker = MouseTracker::new();
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: 20,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };

        let interaction = tracker.handle_event(event, 80, 24, 1.0, 1.0);
        assert!(matches!(
            interaction,
            Some(Interaction::ThermalPulse { .. })
        ));
    }

    #[test]
    fn test_mouse_drag_emits_stir_velocity() {
        let mut tracker = MouseTracker::new();

        // 1. Initial click
        let click_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 20,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let _ = tracker.handle_event(click_event, 80, 24, 1.0, 1.0);

        // 2. Drag to the right and down in terminal (which is negative dy in sim space)
        let drag_event = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 30,
            row: 15,
            modifiers: KeyModifiers::NONE,
        };
        let interaction = tracker.handle_event(drag_event, 80, 24, 1.0, 1.0);

        match interaction {
            Some(Interaction::Stir { vx, vy, .. }) => {
                assert!(vx > 0.0, "Dragging rightwards must produce positive vx");
                assert!(
                    vy < 0.0,
                    "Dragging downwards in terminal must produce negative vy in sim space"
                );
            }
            _ => panic!(
                "Expected Stir interaction from drag event, got {:?}",
                interaction
            ),
        }
    }

    #[test]
    fn test_mouse_scroll_emits_pressure() {
        let mut tracker = MouseTracker::new();
        let up_event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let down_event = MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };

        let up_action = tracker.handle_event(up_event, 80, 24, 1.0, 1.0);
        assert_eq!(up_action, Some(Interaction::Pressure { delta: 1.0 }));

        let down_action = tracker.handle_event(down_event, 80, 24, 1.0, 1.0);
        assert_eq!(down_action, Some(Interaction::Pressure { delta: -1.0 }));
    }
}
