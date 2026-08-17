# LavaTerm Vibecode Prompt 02 — Phase & Issue Execution

## Vai trò

Bạn là AI coding agent phát triển LavaTerm theo từng phase.

Không được tự ý nhảy nhiều phase chỉ vì thấy implementation dễ. Mỗi phase phải đạt Definition of Done trước khi chuyển sang phase tiếp theo.

## Product principle

Mục tiêu:

> Beautiful, smooth, configurable metaball lava running natively inside a terminal.

Không biến project thành bloatware.

Ưu tiên:

```text
visual quality
> clean architecture
> correctness
> performance
> integrations
> feature count
```

---

# Phase 0 — Repository Foundation

### Mục tiêu

Có repository, CI, documentation và architecture skeleton.

### Issues

1. Bootstrap Rust project.
2. Repository documentation.
3. GitHub issue templates.
4. CI.
5. Core architecture skeleton.
6. Configuration schema skeleton.

### Exit criteria

```text
cargo build
cargo test
cargo fmt --check
cargo clippy
```

đều pass.

---

# Phase 1 — Simulation Core

### Mục tiêu

Tạo metaball simulation độc lập terminal.

### Issue 1: Blob model

Implement:

```text
Blob
- position
- velocity
- radius
- temperature
```

Acceptance:

- Không phụ thuộc terminal.
- Có unit tests cho initialization/state.

### Issue 2: Scalar field

Implement field evaluation:

```text
field(x,y) = Σ contribution(blob_i)
```

Acceptance:

- Field tăng khi gần blob.
- Field giảm khi xa blob.
- Deterministic test cases.

### Issue 3: Metaball threshold

Implement thresholding:

```text
field >= threshold
```

Acceptance:

- Một blob tạo được vùng liên tục.
- Hai blob gần nhau có thể tạo merged shape.

### Issue 4: Basic physics

Implement:

```text
gravity
buoyancy
viscosity
```

Acceptance:

- Blob nóng có xu hướng nổi.
- Blob có damping.
- Simulation ổn định trong timestep hợp lệ.

### Issue 5: Natural motion

Thêm noise/turbulence nhẹ.

Không dùng random uncontrolled trong test.

Acceptance:

- Runtime motion organic.
- Unit tests deterministic bằng seeded source nếu cần.

### Phase 1 exit criteria

Có thể chạy simulation mà không cần terminal.

Có test chứng minh:

- field;
- threshold;
- physics;
- timestep;
- deterministic behavior khi seed giống nhau.

---

# Phase 2 — Virtual Canvas

### Mục tiêu

Tách simulation resolution khỏi terminal resolution.

### Issues

1. Framebuffer abstraction.
2. Scalar field → framebuffer.
3. Resize-aware canvas.
4. Frame comparison/double buffering.

### Acceptance

Có pipeline:

```text
Simulation
→ virtual framebuffer
```

Không có `crossterm` trong core.

---

# Phase 3 — Terminal Renderer

### Mục tiêu

Render lava đẹp trong terminal.

### Issue 1: Terminal backend

Dùng `crossterm` để:

- raw mode;
- alternate screen;
- cursor hide/show;
- resize events;
- cleanup.

### Issue 2: Half-block renderer

Render virtual pixels bằng:

```text
▀
▄
```

Acceptance:

- đúng orientation;
- không flicker;
- resize được;
- true color hoạt động.

### Issue 3: Gradient

Implement palette interpolation.

Ví dụ:

```text
temperature
→ palette
→ RGB
```

Không hard-code một theme duy nhất.

### Issue 4: Frame output

Tối ưu:

```text
framebuffer
→ diff
→ ANSI batch
→ stdout
```

Không gọi print cho từng pixel.

### Phase 3 exit criteria

Chạy:

```bash
lavaterm
```

và thấy lava animation thực sự.

MVP visual quality là tiêu chí quan trọng nhất.

---

# Phase 4 — Renderers

### Mục tiêu

Thêm nhiều terminal rendering modes.

### Issues

- Block renderer.
- Braille renderer.
- Renderer trait/interface.
- Renderer selection qua config/CLI.

CLI:

```bash
lavaterm --renderer halfblock
lavaterm --renderer block
lavaterm --renderer braille
```

Acceptance:

- Các renderer dùng cùng framebuffer.
- Không duplicate simulation logic.
- Resize đúng.
- Benchmark từng renderer.

---

# Phase 5 — Configuration

### Mục tiêu

TOML config đủ mạnh nhưng đơn giản.

Schema:

```toml
[simulation]
blobs = 12
gravity = 0.12
buoyancy = 0.8
viscosity = 0.93
noise = 0.15

[render]
renderer = "halfblock"
fps = 30
gradient = true

[palette]
bottom = "#ff3b00"
middle = "#ff7a00"
top = "#7b2cff"
```

Issues:

- Config loading.
- Defaults.
- Validation.
- CLI overrides.
- Example config.

Acceptance:

```bash
lavaterm --config ~/.config/lavaterm/config.toml
```

hoạt động.

Config lỗi phải trả error rõ ràng.

---

# Phase 6 — System Reactive

### Mục tiêu

Biến LavaTerm thành ambient system visualizer.

Signal abstraction:

```text
cpu
memory
battery
disk
network
```

Không để core trực tiếp phụ thuộc vào OS APIs.

Thiết kế:

```text
System Provider
      ↓
Reactive Signals
      ↓
Simulation
```

Mapping ban đầu:

```text
CPU    → turbulence
RAM    → blob size
Disk   → bubble frequency
Battery → temperature
```

Acceptance:

- Chạy được khi provider không khả dụng.
- Linux là target chính.
- Không crash nếu metric missing.
- Có mock provider để test.

---

# Phase 7 — Audio Reactive

### Mục tiêu

Làm LavaTerm phản ứng với nhạc.

Pipeline:

```text
PipeWire
→ PCM
→ FFT
→ spectrum bands
→ reactive signals
→ simulation
```

Signals:

```text
bass
mid
treble
```

Mapping:

```text
bass   → buoyancy
mid    → turbulence
treble → vibration/color
```

Architecture:

```text
AudioSource
      ↓
Spectrum
      ↓
ReactiveInput
```

Core không biết PipeWire tồn tại.

Acceptance:

- Audio capture hoạt động trên Linux.
- FFT có test với synthetic signal.
- Không có audio device vẫn chạy bình thường.
- Audio processing không block render loop.

---

# Phase 8 — Theme Engine

### Mục tiêu

Tích hợp ricing ecosystem.

Targets:

- pywal.
- wallust.
- terminal ANSI palette.

API concept:

```text
Theme Provider
      ↓
Palette
      ↓
Renderer
```

Không hard-code Catppuccin/Tokyo Night vào core.

Có thể hỗ trợ:

```bash
lavaterm --theme auto
lavaterm --theme pywal
```

---

# Phase 9 — tmux / zellij / Widget Mode

### Mục tiêu

LavaTerm usable trong terminal multiplexer.

Modes:

```bash
lavaterm --compact
lavaterm --widget
```

Yêu cầu:

- adaptive resolution;
- no startup noise;
- graceful cleanup;
- stable frame rate;
- correct resize handling.

---

# Phase 10 — Interaction

### Mục tiêu

Cho phép user tác động vào lava.

### Structure

```text
Phase 10
├── 10.1 Mouse click → Shockwave
├── 10.2 Mouse drag → Stirring
└── 10.3 Keyboard → Ripple
```

### Features

```text
mouse click → impact / shockwave
mouse drag  → stir
keyboard    → ripple
scroll      → pressure
```

Chỉ bắt đầu phase này khi rendering và simulation đã ổn định.

---

# Phase 11 — Cross-platform

### Linux

Primary target:

- PipeWire.
- Linux system metrics.

### Windows

Target:

- terminal rendering;
- WASAPI audio;
- Windows system metrics.

### macOS

Optional later:

- CoreAudio;
- system metrics.

Không làm platform-specific abstraction trước khi core API ổn định.

---

# Phase 12 — Polish / V1.0

Issues:

- performance profiling;
- startup latency;
- memory usage;
- renderer benchmark;
- documentation;
- examples;
- config migration;
- error messages;
- release builds;
- packaging.

Release criteria:

```text
cargo test
cargo clippy
cargo fmt --check
```

pass.

Không có known critical bug.

---

# Issue execution protocol

Mỗi khi nhận một GitHub issue:

## 1. Đọc context

Kiểm tra:

- issue;
- parent phase;
- dependencies;
- related code;
- architecture docs;
- existing tests.

Không đoán API nếu codebase đã có abstraction.

## 2. Plan ngắn

Trước khi sửa:

```text
Files to change
Why
Potential risks
Tests
```

## 3. Implement

Chỉ sửa scope cần thiết.

Không refactor unrelated code.

Không thêm feature "tiện thể".

## 4. Test

Chạy tối thiểu:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

Nếu test không phù hợp với issue, giải thích tại sao.

## 5. Review chính mình

Kiểm tra:

- architecture boundaries;
- error handling;
- performance;
- platform assumptions;
- dead code;
- unnecessary dependency.

## 6. Commit

Commit message rõ:

```text
feat(core): implement metaball field
feat(render): add halfblock renderer
fix(render): handle terminal resize
test(core): add field evaluation tests
docs: document renderer architecture
```

Không gom unrelated changes vào một commit.

## 7. Update issue

Ghi:

```text
Implemented
Tests
Files changed
Known limitations
```

Nếu issue chưa hoàn thành, không đánh dấu complete.

---

# Anti-patterns

Không:

- viết fluid simulation phức tạp khi metaball đủ;
- thêm dependency không cần;
- để renderer phụ thuộc physics;
- để core phụ thuộc OS;
- dùng global mutable state nếu không cần;
- render bằng hàng nghìn `print!()`;
- bỏ qua resize;
- bỏ qua terminal cleanup khi panic/error;
- commit generated files;
- implement nhiều phase trong một issue.

---

# Definition of Done cho mọi issue

Một issue chỉ hoàn thành khi:

1. Implementation đúng scope.
2. Tests phù hợp.
3. Format/clippy/build pass.
4. Documentation được cập nhật nếu behavior/API thay đổi.
5. Không tạo regression rõ ràng.
6. Commit có message phù hợp.
7. Issue description/checklist được cập nhật.
