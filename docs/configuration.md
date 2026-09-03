# LavaTerm Configuration Schema

## 1. Overview

LavaTerm is designed with the philosophy of **zero-configuration out of the box** with powerful customization when needed. If no configuration file is provided, beautiful default parameters are automatically used.

LavaTerm resolves configuration files in the following order:
1. **Explicit CLI argument**: `--config <path>` (or `-c <path>`)
2. **Platform-specific auto-discovery path**:
   - **Linux / Unix**: `$XDG_CONFIG_HOME/lavaterm/config.toml` $\to$ `$HOME/.config/lavaterm/config.toml`
   - **macOS**: `$XDG_CONFIG_HOME/lavaterm/config.toml` (if set) $\to$ `$HOME/Library/Application Support/lavaterm/config.toml` $\to$ `$HOME/.config/lavaterm/config.toml`
   - **Windows**: `%APPDATA%\lavaterm\config.toml` $\to$ `%USERPROFILE%\AppData\Roaming\lavaterm\config.toml` $\to$ `%USERPROFILE%\.config\lavaterm\config.toml` $\to$ `$XDG_CONFIG_HOME/lavaterm/config.toml`
3. **Hardcoded Defaults**: Safe built-in parameters if no file exists.

---

## 2. Complete TOML Schema

```toml
# ==========================================
# LavaTerm Configuration File
# ==========================================

[simulation]
# Number of metaball blobs in the chamber
blobs = 12

# Gravitational downward pull (arbitrary simulation units)
gravity = 0.12

# Buoyancy upward force factor for heated blobs
buoyancy = 0.80

# Fluid viscosity drag damping factor (0.0 to 1.0)
viscosity = 0.93

# Subtle thermal Brownian motion noise amplitude
noise = 0.15

# Isosurface metaball threshold for lava boundary
threshold = 1.00

# Thermal transfer rate with chamber boundaries (> 0.0 to 5.0)
thermal_transfer_rate = 0.40

[render]
# Terminal rendering backend: "halfblock" | "block" | "braille"
renderer = "halfblock"

# Target frame rate (FPS)
fps = 30

# Enable 24-bit True Color gradient interpolation
gradient = true

[palette]
# Hex color code for the hot bottom heat zone
bottom = "#ff3b00"

# Hex color code for the middle convective core
middle = "#ff7a00"

# Hex color code for the cooled top chamber
top = "#7b2cff"

# Background color for empty fluid space (Hex or "none" for default terminal background)
background = "#0d0d15"

[reactive]
# Enable ambient system-reactive visualizer mode
enabled = false
# Polling interval for /proc metrics (milliseconds)
poll_interval_ms = 500

[audio]
# Enable audio-reactive visualizer mode
enabled = false
# BPM tempo for synthetic fallback beat generator
bpm = 120.0
# Optional specific audio capture device name
# device = "default"


[theme]
# Active theme preset ("lava", "ocean", "cyberpunk", "synthwave", "nord", "forest", "monochrome", "matrix", "sunset", "dracula", "catppuccin", "tokyo-night"), "auto", "pywal", "wallust", or custom file path
name = "lava"
# path = "/path/to/custom_theme.json"

[widget]
# Force compact geometry scaling
compact = false
# Target FPS when operating in widget mode (default 15)
fps = 15
# Run inline within current terminal without entering alternate screen
inline = false
# Explicit width constraint (columns)
# width = 40
# Explicit height constraint (rows)
# height = 12
# Automatically adapt blob count and physics for small viewports
adapt_blobs = true

[interaction]
# Enable mouse click shockwaves, dragging, and scroll pressure
mouse = true
# Enable keyboard typing ripples
keyboard_ripple = true
# Shockwave force multiplier (0.1 to 10.0)
shockwave_force = 1.0
# Drag stirring force multiplier (0.1 to 10.0)
stir_force = 1.0
```

---

## 3. Configuration Fields Reference

### `[simulation]`
| Field | Type | Default | Description |
|---|---|---|---|
| `blobs` | integer | `12` | Number of simultaneous blobs in the fluid chamber (range: 1..128) |
| `gravity` | float | `0.12` | Downward gravitational acceleration constant |
| `buoyancy` | float | `0.80` | Upward thermal buoyancy acceleration coefficient |
| `viscosity` | float | `0.93` | Velocity retention factor per second |
| `noise` | float | `0.15` | Thermal turbulence and lateral drift amplitude |
| `threshold` | float | `1.00` | Field intensity isosurface threshold |
| `thermal_transfer_rate` | float | `0.40` | Rate of heat exchange at chamber boundaries (> 0.0..5.0) |

### `[render]`
| Field | Type | Default | Description |
|---|---|---|---|
| `renderer` | string | `"halfblock"` | Rendering engine: `"halfblock"` (high-res), `"block"`, or `"braille"` |
| `fps` | integer | `30` | Target render frequency (1..240) |
| `gradient` | boolean | `true` | Interpolate smooth colors across field intensity |

### `[palette]`
| Field | Type | Default | Description |
|---|---|---|---|
| `bottom` | string | `"#ff3b00"` | Hex color for maximum temperature / heat source |
| `middle` | string | `"#ff7a00"` | Hex color for medium temperature |
| `top` | string | `"#7b2cff"` | Hex color for cooled top zone |
| `background` | string | `"#0d0d15"` | Hex color for void fluid chamber background |

### `[theme]`
| Field | Type | Default | Description |
|---|---|---|---|
| `name` | string | `None` | Preset name, `"auto"`, `"pywal"`, `"wallust"`, or theme file path |
| `path` | string | `None` | Explicit path to JSON/TOML theme file |

### `[reactive]`
| Field | Type | Default | Description |
|---|---|---|---|
| `enabled` | boolean | `false` | Enable background system resource polling (CPU/RAM/Battery) |
| `poll_interval_ms` | integer | `500` | Polling frequency in milliseconds |

### `[audio]`
| Field | Type | Default | Description |
|---|---|---|---|
| `enabled` | boolean | `false` | Enable real-time audio FFT capture mode |
| `bpm` | float | `120.0` | Synthetic beat pulse frequency |

### `[widget]`
| Field | Type | Default | Description |
|---|---|---|---|
| `compact` | boolean | `false` | Enable compact viewport mode scaling |
| `fps` | integer | `15` | Frame rate used when running in widget mode |
| `inline` | boolean | `false` | Default to inline rendering without alternate screen |
| `width` | integer | `None` | Viewport width constraint |
| `height` | integer | `None` | Viewport height constraint |
| `adapt_blobs` | boolean | `true` | Dynamically scale blob count and radii based on geometry |

### `[interaction]`
| Field | Type | Default | Description |
|---|---|---|---|
| `mouse` | boolean | `true` | Enable mouse click shockwaves, dragging, and scroll pressure |
| `keyboard_ripple` | boolean | `true` | Enable keyboard typing ripples |
| `shockwave_force` | float | `1.0` | Multiplier for mouse click shockwave force (0.1..10.0) |
| `stir_force` | float | `1.0` | Multiplier for mouse drag stirring force (0.1..10.0) |

