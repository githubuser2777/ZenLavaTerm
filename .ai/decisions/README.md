# Architectural Decision Records (ADRs)

This directory documents key architectural, structural, and technical decisions made in ZenLavaTerm using a lightweight ADR format.

---

## 1. ADR Lifecycle & Template

Decisions are immutable once accepted. If a decision is superseded, its status is changed to `Superseded by ADR-XXXX`, and a new ADR is created.

### Lightweight ADR Template
```markdown
# ADR-XXXX: <Decision Title>

- **Status**: [ Proposed | Accepted | Deprecated | Superseded ]
- **Date**: YYYY-MM-DD
- **Author(s)**: <Name or Role>
- **Context**: What problem are we solving? What constraints or requirements apply?
- **Decision**: What is the chosen technical approach?
- **Consequences**:
  - **Positive**: What benefits do we gain?
  - **Negative / Trade-offs**: What complexity or trade-offs are introduced?
  - **Compliance / Invariants**: What rules must all future changes adhere to?
```

---

## 2. Decision Index

| ADR ID | Title | Date | Status |
|---|---|:---:|:---:|
| [ADR-0001](file:///home/skids/Documents/code/ZenLavaTerm/.ai/decisions/0001-unidirectional-simulation-pipeline.md) | Unidirectional Simulation & Rendering Pipeline | 2026-08-10 | **Accepted** |
| [ADR-0002](file:///home/skids/Documents/code/ZenLavaTerm/.ai/decisions/0002-lock-free-spsc-seqlock-audio-ringbuffer.md) | Lock-Free SPSC Seqlock Audio Ring Buffer | 2026-08-20 | **Accepted** |
| [ADR-0003](file:///home/skids/Documents/code/ZenLavaTerm/.ai/decisions/0003-terminal-native-crossterm-ui.md) | Terminal-Native TUI vs GUI / Tauri Frameworks | 2026-08-15 | **Accepted** |
| [ADR-0004](file:///home/skids/Documents/code/ZenLavaTerm/.ai/decisions/0004-native-serde-aliases-for-config-evolution.md) | Native Serde Field Aliasing for Backward Compatibility | 2026-09-03 | **Accepted** |
