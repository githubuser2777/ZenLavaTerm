# LavaTerm Vibecode Prompt 01 — Project Setup

## Mục tiêu

Bạn là AI coding agent chịu trách nhiệm khởi tạo project **LavaTerm**, một terminal-native ambient visualizer viết bằng Rust.

Hãy thiết lập repository theo hướng production-minded ngay từ đầu, nhưng tuyệt đối không over-engineer implementation. Giai đoạn này chủ yếu tạo nền móng: Git repo, issue tracking, project structure, documentation, conventions và skeleton build được.

## Product context

LavaTerm là một terminal visualizer lấy lava lamp/metaball làm core visual.

Core concept:

```text
Signals
  ├── audio
  ├── system
  ├── input
  └── time
        ↓
Lava Simulation
        ↓
Virtual Canvas
        ↓
Terminal Renderer
        ↓
TTY
```

MVP chưa cần audio, system monitoring hay mouse interaction. Core trước:

```text
metaball simulation
→ terminal rendering
→ true-color gradient
→ configurable TOML
```

## Tech direction

Ưu tiên:

- Rust stable.
- Cargo.
- `crossterm` cho terminal control.
- Custom renderer thay vì phụ thuộc Ratatui cho core rendering.
- TOML configuration.
- Cross-platform architecture ngay từ đầu.
- Linux là platform phát triển chính.
- Windows compatibility không được phá kiến trúc.

Không được thêm dependency chỉ vì "có thể hữu ích". Mỗi dependency phải có lý do.

## 1. Git repository

Tạo repository Git sạch.

Thiết lập:

- `main` là stable branch.
- `.gitignore` phù hợp Rust.
- `LICENSE` nếu chưa được chỉ định thì dùng placeholder rõ ràng hoặc hỏi trước khi chọn license.
- `README.md`.
- `CHANGELOG.md`.
- `CONTRIBUTING.md`.
- `CODE_OF_CONDUCT.md` nếu phù hợp.
- `SECURITY.md` nếu project public.
- `.github/ISSUE_TEMPLATE/`.
- `.github/PULL_REQUEST_TEMPLATE.md`.

Không commit build artifacts, IDE state hoặc machine-specific files.

## 2. Repository structure

Tạo skeleton theo hướng:

```text
lavaterm/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/
│
├── docs/
│   ├── architecture.md
│   ├── roadmap.md
│   ├── rendering.md
│   ├── simulation.md
│   ├── configuration.md
│   └── contributing.md
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── physics.rs
│   │   ├── metaball.rs
│   │   ├── field.rs
│   │   └── simulation.rs
│   │
│   ├── render/
│   │   ├── mod.rs
│   │   ├── framebuffer.rs
│   │   ├── color.rs
│   │   ├── halfblock.rs
│   │   └── block.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── schema.rs
│   │
│   └── input/
│       ├── mod.rs
│       └── keyboard.rs
│
├── tests/
├── examples/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
└── LICENSE
```

Nếu một module chưa được implement, tạo skeleton/documentation rõ ràng thay vì viết fake implementation.

## 3. Kiến trúc

Thiết kế `core` độc lập với terminal.

Core không được import `crossterm`.

Simulation chỉ biết:

- simulation state;
- blobs;
- physics;
- field;
- time delta;
- reactive signals nếu sau này abstraction được thêm.

Renderer nhận virtual canvas/framebuffer và chuyển nó thành terminal cells.

Kiến trúc mục tiêu:

```text
Input / Signals
      ↓
Simulation
      ↓
Virtual Framebuffer
      ↓
Renderer
      ↓
Terminal Backend
```

Không để terminal resolution trực tiếp quyết định physics.

## 4. Documentation

README phải giải thích ngắn gọn:

- LavaTerm là gì.
- Screenshot/GIF placeholder nếu chưa có.
- Quick start.
- Basic usage.
- Configuration.
- Development.
- Roadmap.

`docs/architecture.md` phải mô tả dependency direction.

`docs/simulation.md` mô tả:

- metaball;
- scalar field;
- threshold;
- buoyancy;
- gravity;
- viscosity;
- noise;
- timestep.

`docs/rendering.md` mô tả:

- virtual canvas;
- terminal cell;
- half-block;
- block;
- true color;
- buffering;
- dirty-frame optimization.

`docs/configuration.md` mô tả schema TOML dự kiến.

`docs/roadmap.md` chứa roadmap phase-level, không viết task implementation quá chi tiết.

## 5. Git Issues

Tạo GitHub Issues cho các milestone đầu tiên.

Issue phải nhỏ, độc lập và có acceptance criteria.

Tạo tối thiểu:

- Project bootstrap.
- Core data model.
- Metaball field.
- Basic physics.
- Virtual framebuffer.
- Half-block renderer.
- True-color gradient.
- Main render loop.
- TOML configuration.
- Resize handling.
- Performance benchmark.
- Documentation.
- CI.

Mỗi issue có:

```text
Goal
Context
Scope
Non-goals
Acceptance Criteria
Technical Notes
Dependencies
```

Không tạo issue kiểu "Build everything".

## 6. CI

Thiết lập CI tối thiểu:

- `cargo fmt --check`
- `cargo clippy`
- `cargo test`
- `cargo build`

Nếu thêm cross-platform matrix thì chỉ làm khi không làm workflow phức tạp quá mức.

## 7. Development quality

Thiết lập:

- `rustfmt`.
- Clippy policy hợp lý.
- Unit test structure.
- Integration test structure.
- Documentation comments cho public API.
- Error handling rõ ràng.
- Không dùng `unwrap()` tùy tiện trong production path.

## 8. Important constraints

Không implement audio, PipeWire, system monitoring, mouse interaction hoặc advanced fluid simulation trong setup phase.

Không dùng placeholder abstraction quá mức.

Không tạo framework nội bộ khi chưa có nhu cầu.

Không tối ưu premature.

Mọi quyết định kiến trúc phải phục vụ MVP:

```text
Beautiful metaball lava in a terminal.
```

## 9. Definition of Done

Setup phase hoàn thành khi:

- Repository build được.
- CI chạy được.
- Project structure rõ ràng.
- README có thể hướng dẫn developer mới chạy project.
- Architecture docs tồn tại.
- Roadmap tồn tại.
- Git issues được tạo.
- Không có dead dependency.
- Không có module giả vờ đã implement.
- Có một minimal executable skeleton chạy được.

Sau khi hoàn thành, báo cáo:

1. Files created.
2. Dependencies.
3. Git commits.
4. Issues created.
5. CI status.
6. Architecture decisions.
7. Những gì cố tình chưa implement.
