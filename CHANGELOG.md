# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Phase 0 repository foundation and project bootstrap.
- Core decoupled architecture with `core`, `render`, `config`, and `input` modules.
- Metaball mathematical scalar field evaluation and physics simulation skeleton.
- Virtual Framebuffer abstraction with RGB color interpolation and palette mapping.
- Half-block (`▀`) and Block (`█`) terminal renderer abstractions.
- Braille dot matrix (`U+2800`..`U+28FF`) 2x4 sub-pixel terminal renderer (`BrailleRenderer`).
- TOML configuration schema with serde validation and default fallback.
- CLI argument parsing supporting configuration overrides, renderer selection (`halfblock`, `block`, `braille`), and headless testing.
- Panic hook and terminal cleanup handlers.
- Criterion micro-benchmark suite (`benches/field_and_render.rs`) evaluating field potential and renderer serialization throughput.
- Automated multi-platform GitHub Release workflow (`.github/workflows/release.yml`) targeting Linux, macOS, and Windows with SHA256 checksums.
- Comprehensive documentation suite: `architecture.md`, `simulation.md`, `rendering.md`, `configuration.md`, `roadmap.md`, `contributing.md`.
- GitHub issue templates, PR template, and GitHub Actions CI workflow (`cargo fmt`, `cargo clippy`, `cargo test`, `cargo build`, `cargo bench --no-run`).
