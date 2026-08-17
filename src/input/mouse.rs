//! Mouse event tracking and translation into domain interactions.

use super::coords::terminal_to_sim_coords;
use crate::core::interaction::Interaction;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use std::time::Instant;

/// Stateful mouse tracker that converts mouse movements and gestures into fluid interactions.
#[derive(Debug, Clone)]
pub struct MouseTracker {
    /// Previous continuous position of the mouse during dragging.
    last_drag_pos: Option<(f32, f32)>,
    /// Timestamp of previous drag event for cadence-normalized velocity calculation.
    last_drag_time: Option<Instant>,
}

impl Default for MouseTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseTracker {
    /// Creates a new empty mouse tracker.
    pub fn new() -> Self {
        Self {
            last_drag_pos: None,
            last_drag_time: None,
        }
    }

    /// Processes a crossterm `MouseEvent` and returns an optional domain `Interaction` using current time.
    pub fn handle_event(
        &mut self,
        event: MouseEvent,
        cols: u16,
        rows: u16,
        shockwave_force: f32,
        stir_force: f32,
    ) -> Option<Interaction> {
        self.handle_event_at(
            event,
            cols,
            rows,
            shockwave_force,
            stir_force,
            Instant::now(),
        )
    }

    /// Processes a crossterm `MouseEvent` at a specific timestamp for deterministic velocity derivation.
    pub fn handle_event_at(
        &mut self,
        event: MouseEvent,
        cols: u16,
        rows: u16,
        shockwave_force: f32,
        stir_force: f32,
        now: Instant,
    ) -> Option<Interaction> {
        let (sim_x, sim_y) = terminal_to_sim_coords(event.column, event.row, cols, rows);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.last_drag_pos = Some((sim_x, sim_y));
                self.last_drag_time = Some(now);
                Some(Interaction::Shockwave {
                    x: sim_x,
                    y: sim_y,
                    force: shockwave_force,
                })
            }
            MouseEventKind::Down(MouseButton::Right) => {
                // Right click injects thermal pulse and does NOT affect left-button drag tracking
                Some(Interaction::ThermalPulse {
                    x: sim_x,
                    y: sim_y,
                    temperature_delta: 0.60,
                    radius: 0.20,
                })
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some((last_x, last_y)) = self.last_drag_pos {
                    let dt = if let Some(last_time) = self.last_drag_time {
                        now.duration_since(last_time)
                            .as_secs_f32()
                            .clamp(0.005, 0.20)
                    } else {
                        0.033
                    };
                    self.last_drag_pos = Some((sim_x, sim_y));
                    self.last_drag_time = Some(now);

                    let dx = sim_x - last_x;
                    let dy = sim_y - last_y;

                    // Physical pointer velocity (dx/dt, dy/dt) scaled by stir_force
                    let raw_vx = (dx / dt) * 0.15 * stir_force;
                    let raw_vy = (dy / dt) * 0.15 * stir_force;
                    let vx = raw_vx.clamp(-2.0, 2.0);
                    let vy = raw_vy.clamp(-2.0, 2.0);

                    Some(Interaction::Stir {
                        x: sim_x,
                        y: sim_y,
                        vx,
                        vy,
                        radius: 0.22,
                    })
                } else {
                    self.last_drag_pos = Some((sim_x, sim_y));
                    self.last_drag_time = Some(now);
                    None
                }
            }
            MouseEventKind::Up(_) => {
                self.last_drag_pos = None;
                self.last_drag_time = None;
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
        self.last_drag_time = None;
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

    #[test]
    fn test_right_click_does_not_set_drag_position() {
        let mut tracker = MouseTracker::new();

        // Right click down
        let right_down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let _ = tracker.handle_event(right_down, 80, 24, 1.0, 1.0);

        // Immediate left drag without prior left click down should initialize drag position rather than producing phantom delta
        let left_drag = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 20,
            row: 20,
            modifiers: KeyModifiers::NONE,
        };
        let action = tracker.handle_event(left_drag, 80, 24, 1.0, 1.0);
        assert_eq!(
            action, None,
            "Initial drag after right-click must initialize position and not emit phantom velocity"
        );
    }

    #[test]
    fn test_drag_velocity_cadence_normalization() {
        use std::time::Duration;

        let start_time = Instant::now();

        // Scenario A: 100ms elapsed with 20 columns displacement (20/80 = 0.25 in x)
        let mut tracker_a = MouseTracker::new();
        let click_a = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let _ = tracker_a.handle_event_at(click_a, 80, 24, 1.0, 1.0, start_time);

        let drag_a = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 30,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let action_a = tracker_a.handle_event_at(
            drag_a,
            80,
            24,
            1.0,
            1.0,
            start_time + Duration::from_millis(100),
        );

        // Scenario B: 50ms elapsed with 10 columns displacement (10/80 = 0.125 in x) => Same physical pointer speed!
        let mut tracker_b = MouseTracker::new();
        let click_b = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let _ = tracker_b.handle_event_at(click_b, 80, 24, 1.0, 1.0, start_time);

        let drag_b = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 20,
            row: 10,
            modifiers: KeyModifiers::NONE,
        };
        let action_b = tracker_b.handle_event_at(
            drag_b,
            80,
            24,
            1.0,
            1.0,
            start_time + Duration::from_millis(50),
        );

        if let (
            Some(Interaction::Stir { vx: vx_a, .. }),
            Some(Interaction::Stir { vx: vx_b, .. }),
        ) = (action_a, action_b)
        {
            assert!(
                (vx_a - vx_b).abs() < 1e-4,
                "Both drag scenarios have identical pointer velocity, so vx must match: {} vs {}",
                vx_a,
                vx_b
            );
        } else {
            panic!("Expected both actions to produce Stir interactions");
        }
    }
}
