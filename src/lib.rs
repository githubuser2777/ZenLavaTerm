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
#[derive(Debug)]
pub enum LavaError {
    /// Standard I/O or terminal communication error.
    Io(std::io::Error),

    /// Configuration parsing or validation error.
    Config(String),

    /// Rendering or framebuffer error.
    Render(String),

    /// Simulation or physics error.
    Simulation(String),

    /// Audio or FFT processing error.
    Audio(String),
}

impl fmt::Display for LavaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Config(msg) => write!(f, "Configuration error: {msg}"),
            Self::Render(msg) => write!(f, "Rendering error: {msg}"),
            Self::Simulation(msg) => write!(f, "Simulation error: {msg}"),
            Self::Audio(msg) => write!(f, "Audio error: {msg}"),
        }
    }
}

impl std::error::Error for LavaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LavaError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
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
