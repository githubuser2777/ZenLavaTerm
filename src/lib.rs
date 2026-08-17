//! LavaTerm — A terminal-native ambient lava lamp & metaball visualizer.
//!
//! This crate provides the decoupled core simulation, virtual framebuffer,
//! color processing, and terminal rendering abstractions for LavaTerm.

pub mod audio;
pub mod config;
pub mod core;
pub mod input;
pub mod reactive;
pub mod render;
pub mod theme;
pub mod widget;

use std::fmt;

/// Top-level error types for LavaTerm operations.
#[derive(Debug, thiserror::Error)]
pub enum LavaError {
    /// Standard I/O or terminal communication error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration parsing or validation error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Rendering or framebuffer error.
    #[error("Rendering error: {0}")]
    Render(String),

    /// Simulation or physics error.
    #[error("Simulation error: {0}")]
    Simulation(String),

    /// Audio or FFT processing error.
    #[error("Audio error: {0}")]
    Audio(String),
}

/// Specialized Result type for LavaTerm operations.
pub type Result<T> = std::result::Result<T, LavaError>;

/// Color parsing error for hex strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError(pub String);

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid hex color code: {}", self.0)
    }
}

impl std::error::Error for ParseColorError {}
