//! Core simulation layer decoupled completely from terminal rendering.

pub mod field;
pub mod metaball;
pub mod physics;
pub mod simulation;

pub use field::ScalarField;
pub use metaball::Blob;
pub use physics::PhysicsParams;
pub use simulation::Simulation;
