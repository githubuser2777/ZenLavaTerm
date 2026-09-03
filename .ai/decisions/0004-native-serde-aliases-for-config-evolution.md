# ADR-0004: Native Serde Field Aliasing for Backward-Compatible Configuration

- **Status**: Accepted
- **Date**: 2026-09-03
- **Context**: 
  Over several phases of development, configuration keys evolved (e.g., `num_blobs` -> `blobs`, `target_fps` -> `fps`, `smooth_gradient` -> `gradient`). Previously, a custom AST-rewriting migration engine (`src/config/migrate.rs`) walked TOML AST trees to rewrite deprecated keys. This added 150+ lines of complex parsing logic, extra runtime overhead, and potential edge-case parsing bugs.
- **Decision**:
  Remove `src/config/migrate.rs` and leverage native Serde field aliases (`#[serde(alias = "...")]`) directly on configuration schema structs in `src/config/schema.rs`.
- **Consequences**:
  - **Positive**: Removed ~150 lines of bespoke parsing code; leverages Serde's heavily audited, zero-cost deserialization; seamlessly parses both modern and legacy TOML configurations without manual migration passes.
  - **Negative / Trade-offs**: Deprecated keys are silently accepted during deserialization rather than being rewritten on disk, which is preferred as it avoids unexpected file mutation.
  - **Invariants**: Keep historical aliases (`alias = "num_blobs"`, etc.) active in `src/config/schema.rs` to ensure continuous backward compatibility with user configuration files.
