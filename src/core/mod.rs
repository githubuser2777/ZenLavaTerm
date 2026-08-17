//! Core simulation layer decoupled completely from terminal rendering.

pub mod field;
pub mod interaction;
pub mod metaball;
pub mod physics;
pub mod simulation;

pub use field::ScalarField;
pub use interaction::{
    apply_pressure, apply_ripple, apply_shockwave, apply_stir, apply_thermal_pulse, Interaction,
};
pub use metaball::Blob;
pub use physics::PhysicsParams;
pub use simulation::Simulation;
