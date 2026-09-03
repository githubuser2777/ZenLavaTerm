---
name: Development Task
about: General development task, refactor, performance optimization, or release work
title: '[TASK] '
labels: ['task']
assignees: ''
---

## Objective
Define the goal of this task clearly.

## Subsystem & Scope
- [ ] `core` (metaballs, physics, potential field)
- [ ] `audio` (CPAL, ring buffer, FFT spectrum)
- [ ] `reactive` (Linux/Windows/macOS system telemetry)
- [ ] `render` (framebuffer, halfblock, block, braille)
- [ ] `input` (crossterm keyboard, mouse interactions)
- [ ] `config` / `theme` (TOML schema, palette parsers)
- [ ] `widget` (compact scaling, snapshot mode)
- [ ] `packaging` / `ci` / `docs`

## Acceptance Criteria
- [ ] Implementation complete with zero panics (`Result<T, LavaError>`)
- [ ] Unit tests added / updated
- [ ] Integration tests pass (`cargo test`)
- [ ] Headless validation passes (`cargo run -- --headless --frames 30`)
- [ ] `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings` pass
- [ ] Documentation updated in `docs/` or `README.md` if applicable
