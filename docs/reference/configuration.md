# Configuration Reference

ZenLavaTerm supports rich configuration via a TOML configuration file. A ready-to-copy sample configuration file is available at [docs/configuration.md](file:///home/skids/Documents/code/ZenLavaTerm/docs/configuration.md).

---

## 1. Default File Discovery Order

ZenLavaTerm automatically locates `config.toml` in standard platform directories unless overridden via `--config <PATH>`:

- **Linux / BSD**:
  1. `$XDG_CONFIG_HOME/lavaterm/config.toml`
  2. `~/.config/lavaterm/config.toml`
- **macOS**:
  1. `~/Library/Application Support/lavaterm/config.toml`
  2. `~/.config/lavaterm/config.toml`
- **Windows**:
  1. `%APPDATA%\lavaterm\config.toml`
  2. `%USERPROFILE%\.config\lavaterm\config.toml`

---

## 2. Configuration Schema & Tables

### `[window]`
Controls terminal frame rate and renderer preferences:
- `renderer` (string, default: `"halfblock"`): `"halfblock"`, `"block"`, or `"braille"`.
- `fps` (integer, default: `60`): Target frames per second. *(Legacy alias: `target_fps`)*
- `compact` (boolean, default: `false`): Enable compact geometry scaling. *(Legacy alias: `compact_mode`)*
- `gradient` (boolean, default: `true`): Use smooth color interpolation. Set `false` for stepped optimization. *(Legacy alias: `smooth_gradient`)*

### `[simulation]`
Controls physics parameters and blob counts:
- `blobs` (integer, default: `12`): Initial blob count. *(Legacy alias: `num_blobs`)*
- `gravity` (float, default: `0.25`): Gravitational acceleration.
- `viscosity` (float, default: `0.08`): Fluid damping drag coefficient.
- `heat_transfer` (float, default: `0.05`): Thermal exchange rate between fluid and lamp boundaries.

### `[audio]`
Controls audio reactive visualization:
- `enabled` (boolean, default: `false`): Activate audio processing.
- `bpm` (float, default: `120.0`): Fallback synthetic beat frequency. *(Legacy alias: `tempo`)*
- `device` (optional string): Name of explicit audio capture endpoint.
- `loopback` (boolean, default: `false`): Capture system audio mix (WASAPI loopback on Windows).

### `[reactive]`
Controls system resource telemetry:
- `enabled` (boolean, default: `false`): Enable CPU/RAM/battery monitoring.
- `poll_interval_ms` (integer, default: `500`): Background metrics polling rate.

### `[palette]`
Defines active theme colors (hex strings):
- `background` (hex string, e.g. `"#0a0514"`)
- `color1` .. `color5` (hex strings, e.g. `"#ff3366"`, `"#ff9933"`)
