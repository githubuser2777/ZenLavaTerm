# ZenLavaTerm Active Tasks

This file tracks the task(s) currently being executed by developers or AI coding agents. Keep active tasks limited (typically 1 task at a time) to prevent context fragmentation.

---

## Active Task

### [INIT-001] Professional AI-Coding and Documentation Workspace Setup
- **Assignee**: AI Agent (Pair Programming)
- **Goal**: Establish a standardized, repository-wide AI workspace and documentation infrastructure without changing application runtime behavior.
- **Scope**:
  - `AGENTS.md` and `CLAUDE.md` entry points
  - `.cursor/rules/` (`architecture.mdc`, `rust.mdc`, `testing.mdc`, `documentation.mdc`, `release.mdc`, `security.mdc`)
  - `.ai/` (`context/`, `tasks/`, `decisions/`, `prompts/`)
  - `docs/` (`README.md`, `architecture/`, `development/`, `testing/`, `operations/`, `troubleshooting/`, `reference/`, `releases/`)
  - `.github/` templates alignment
- **Status**: Completed
- **Checklist**:
  - [x] Inspect existing repository, architecture, and configuration
  - [x] Create `AGENTS.md` and `CLAUDE.md`
  - [x] Create `.cursor/rules/` suite
  - [x] Create `.ai/context/` and `.ai/tasks/`
  - [x] Create `.ai/decisions/` and `.ai/prompts/`
  - [x] Create structured `docs/` suite
  - [x] Align `.github/` templates
  - [x] Validate tests, formatting, and link integrity
