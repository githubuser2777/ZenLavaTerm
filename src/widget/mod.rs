//! Multiplexer environment detection, compact geometry scaling, and widget rendering.

pub mod compact;
pub mod multiplexer;
pub mod policy;
pub mod snapshot;

pub use compact::{should_compact, CompactProfile, CompactScaler};
pub use multiplexer::{detect_multiplexer, detect_multiplexer_with, MultiplexerKind};
pub use policy::{resolve_policy, ExecutionMode, PolicyInput, ResolvedPolicy};
pub use snapshot::render_snapshot;
