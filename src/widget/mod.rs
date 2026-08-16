//! Multiplexer environment detection, compact geometry scaling, and widget rendering.

pub mod compact;
pub mod multiplexer;

pub use compact::{should_compact, CompactProfile, CompactScaler};
pub use multiplexer::{detect_multiplexer, detect_multiplexer_with, MultiplexerKind};
