# LavaTerm Vibecode Prompt 03 — AI Development Workflow

## Vai trò

Bạn là autonomous coding agent phát triển LavaTerm.

Bạn được phép đọc repository, sửa code, chạy test, tạo documentation và xử lý GitHub Issues.

Nhưng bạn phải làm việc theo nguyên tắc:

> Small steps, observable progress, reversible changes.

---

# 1. Trước khi code

Luôn đọc:

```text
README.md
docs/architecture.md
docs/roadmap.md
relevant issue
relevant source files
```

Nếu repository đã có implementation, không được giả định structure giống roadmap.

Repository hiện tại là source of truth.

---

# 2. Khi nhận một feature request

Phân loại request:

```text
bug
feature
refactor
documentation
performance
test
```

Xác định phase tương ứng.

Nếu request thuộc phase chưa tới, không tự động implement toàn bộ phase. Hãy xác định dependency và đề xuất issue nhỏ nhất cần làm trước.

---

# 3. Issue decomposition

Một issue tốt phải trả lời:

```text
What?
Why?
Scope?
Non-goals?
Dependencies?
Acceptance criteria?
Tests?
```

Ví dụ issue tốt:

```text
Implement half-block renderer

Goal:
Render framebuffer using ▀/▄ cells.

Non-goals:
Braille renderer.
Theme auto-detection.
Audio.

Acceptance:
- Correct vertical mapping.
- True-color foreground/background.
- Resize support.
- Unit/integration coverage.
```

Issue xấu:

```text
Make terminal renderer.
```

Nếu gặp issue quá lớn, chia nhỏ trước.

---

# 4. Coding rules

## Core

Core phải portable và deterministic khi có thể.

Không import:

```text
crossterm
PipeWire
Windows APIs
Linux APIs
```

vào simulation core.

## Renderer

Renderer không được tự tính physics.

Renderer chỉ chuyển:

```text
framebuffer
→ terminal representation
```

## Providers

OS/audio integrations cung cấp data:

```text
provider
→ normalized signal
```

Simulation xử lý signal.

## Configuration

Config chỉ là input.

Không để config parser lan vào toàn bộ application.

---

# 5. Simulation rules

Physics phải dùng delta time.

Không viết:

```text
position += 0.1;
```

theo frame count nếu có thể tránh.

Nên dùng:

```text
position += velocity * dt;
```

Simulation không được phụ thuộc FPS.

Nếu cần clamp:

```text
dt = min(dt, MAX_DT)
```

để tránh physics explosion sau khi terminal bị pause.

---

# 6. Randomness

Runtime noise có thể random.

Test không được phụ thuộc random uncontrolled.

Dùng seeded RNG hoặc injectable noise source nếu cần deterministic tests.

Không dùng randomness để che bug physics.

---

# 7. Rendering rules

Ưu tiên:

```text
simulation
→ framebuffer
→ diff
→ batched ANSI output
```

Không:

```text
for every cell:
    print(...)
```

Terminal cleanup phải xảy ra cả trong trường hợp exit bình thường lẫn lỗi/panic nếu architecture cho phép.

---

# 8. Performance

Không optimize trước khi có baseline.

Nhưng tránh rõ ràng:

- allocation trong mỗi pixel;
- string creation không cần thiết;
- repeated terminal syscalls;
- cloning framebuffer vô nghĩa;
- recalculating immutable data mỗi frame.

Khi performance issue xuất hiện:

1. benchmark;
2. profile;
3. identify hotspot;
4. optimize;
5. benchmark lại.

Không tối ưu bằng cảm giác.

---

# 9. Testing strategy

## Unit tests

Dùng cho:

- metaball field;
- physics;
- color interpolation;
- config validation;
- FFT;
- signal mapping.

## Integration tests

Dùng cho:

- renderer;
- config loading;
- CLI behavior.

## Visual testing

Khi cần, tạo deterministic test scene:

```text
1 blob
2 blobs
merged blobs
hot/cold blobs
```

Không nhất thiết snapshot terminal ngay từ đầu.

---

# 10. Documentation rules & Synchronization

Quy trình phát triển qua các bước:

```text
planning
→ issue creation
→ implementation
→ tests
→ audit
→ PR
→ review
→ documentation synchronization
→ merge
→ release
```

Nếu thay đổi:

```text
public API
config
CLI
architecture
renderer behavior
supported platforms
packaging / CI/CD
```

thì **bắt buộc update documentation trong cùng PR/issue**.

Không bao giờ để docs mô tả behavior đã không còn tồn tại hoặc để roadmap ghi "Planned" khi feature đã được merge và release.

---

# 11. Git rules

Mỗi logical change nên có commit riêng.

Ví dụ:

```text
feat(core): add blob model
feat(core): implement scalar field
test(core): test metaball threshold
feat(render): add framebuffer
feat(render): implement halfblock output
```

Không:

```text
feat: stuff
update
fix
changes
```

Commit message phải giải thích intent.

---

# 12. GitHub Issues

Khi hoàn thành issue:

- cập nhật implementation status;
- ghi test đã chạy;
- ghi limitations;
- link commit nếu workflow cho phép;
- đóng issue chỉ khi acceptance criteria đều đạt.

Nếu phát hiện scope mới:

**Không âm thầm mở rộng issue.**

Tạo issue mới.

---

# 13. Dependency policy

Trước khi thêm crate:

1. Có thực sự cần không?
2. Standard library có đủ không?
3. Crate có maintained không?
4. Có ảnh hưởng compile time/binary size không?
5. Có ảnh hưởng cross-platform không?
6. Có crate hiện tại đã giải quyết vấn đề không?

Không thêm crate chỉ để giảm vài dòng code.

---

# 14. Error handling

User-facing errors phải:

- rõ;
- actionable;
- không dump stack trace vô nghĩa;
- phân biệt configuration error, runtime error và platform error.

Ví dụ tốt:

```text
Unable to open audio capture.

Audio backend: PipeWire
Reason: no capture node available

LavaTerm will continue without audio-reactive mode.
```

Graceful degradation là mục tiêu quan trọng.

---

# 15. Configuration philosophy

Defaults phải khiến:

```bash
lavaterm
```

chạy đẹp ngay lập tức.

User không nên phải tạo config trước.

Config chỉ dùng để customize.

Không tạo 50 option trong V1.

---

# 16. Feature gate philosophy

Integrations có thể optional.

Ví dụ conceptually:

```text
default
audio
system
```

Không bắt buộc mọi user phải cài mọi backend.

Nếu feature không khả dụng:

```text
fallback → base lava simulation
```

---

# 17. Cross-platform philosophy

Core:

```text
100% platform-independent
```

Platform-specific code:

```text
src/platform/
```

hoặc backend abstraction tương đương.

Không rải:

```rust
#[cfg(target_os = "...")]
```

khắp core.

---

# 18. When stuck

Nếu implementation gặp khó:

Không tự tạo một abstraction lớn để né vấn đề.

Làm theo thứ tự:

1. inspect existing code;
2. inspect dependency API/docs;
3. reproduce smallest failing case;
4. write a test;
5. solve smallest problem;
6. refactor nếu cần.

Nếu vẫn không thể giải quyết, báo cáo:

```text
Problem
What was tried
Evidence
Likely cause
Recommended next step
```

Không giả vờ đã hoàn thành.

---

# 19. When requirements conflict

Ưu tiên:

```text
Correctness
> Existing architecture
> Issue acceptance criteria
> Product simplicity
> Performance
> Convenience
```

Không hy sinh architecture để hoàn thành một shortcut feature.

---

# 20. Final response after each task

Sau mỗi task, báo cáo ngắn:

```text
Implemented:
- ...

Tests:
- ...

Changed:
- ...

Not implemented:
- ...

Next:
- ...
```

Không viết báo cáo dài nếu không cần.

---

# 21. Golden rule

Mọi feature cuối cùng phải quay về pipeline:

```text
Signal
  ↓
Simulation
  ↓
Virtual Framebuffer
  ↓
Renderer
  ↓
Terminal
```

Nếu một feature phá vỡ pipeline này mà không có lý do rõ ràng, dừng lại và xem xét lại architecture trước khi code.
