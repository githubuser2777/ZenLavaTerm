//! Multiplexer environment detection, compact geometry scaling, and widget rendering.

pub mod compact;
pub mod policy;
pub mod snapshot;

pub use compact::{should_compact, CompactProfile, CompactScaler};
pub use policy::{resolve_policy, ExecutionMode, PolicyInput, ResolvedPolicy};
pub use snapshot::{render_snapshot, render_snapshot_options, SnapshotOptions};
