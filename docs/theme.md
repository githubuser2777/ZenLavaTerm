# LavaTerm Theme Engine

## 1. Overview

LavaTerm includes a modular **Theme Engine** (Phase 8) designed to fit naturally into Linux desktop ricing and ambient terminal customization setups.

The Theme Engine decouples color palette selection and scheme extraction from terminal rendering and physics. It supports:
1. **Built-in Curated Presets**: Rich, hand-tuned aesthetics (`lava`, `ocean`, `cyberpunk`, `synthwave`, `nord`, `forest`, `monochrome`, `matrix`, `sunset`, `dracula`, `catppuccin`, `tokyo-night`).
2. **Pywal Desktop Color Extraction**: Real-time extraction from `~/.cache/wal/colors.json` or `~/.cache/wal/colors`.
3. **Wallust Dynamic Color Extraction**: Real-time extraction from `~/.cache/wallust/colors.json` or `nix-colors.json`.
4. **Auto-Detection Engine**: Automatic discovery of active desktop color schemes with seamless fallback (`--theme auto`).
5. **Custom Theme Files**: User-provided JSON, TOML, or flat 16-color files.

---

## 2. Architecture & Data Flow

```text
+-------------------------------------------------------------+
|                     Theme Specifications                    |
|      (CLI --theme, TOML [theme], Pywal, Wallust, Auto)      |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                      Theme Engine API                       |
|               (resolve_theme / ThemeProvider)               |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                        ColorPalette                         |
|   (bottom: Rgb, middle: Rgb, top: Rgb, background: Rgb)     |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                 Virtual Framebuffer & Renderer              |
|        (Lava temperature gradient & edge glow blending)     |
+-------------------------------------------------------------+
```

---

## 3. Built-in Preset Themes

LavaTerm includes curated presets:

| Preset Name | Description | Key Hex Colors (Bottom / Middle / Top / Background) |
|---|---|---|
| `lava` (default) | Warm fiery embers & violet apex | `#ff3b00` / `#ff7a00` / `#7b2cff` / `#0d0d15` |
| `ocean` | Bioluminescent deep abyss & neon aqua | `#00f0ff` / `#0077be` / `#0a192f` / `#020b14` |
| `cyberpunk` | High-contrast neon yellow & hot magenta | `#fcee0a` / `#ff0055` / `#7122fa` / `#05050d` |
| `synthwave` | 80s outrun laser cyan & twilight pink | `#ff2a85` / `#9a48d0` / `#2de2e6` / `#120b22` |
| `nord` | Arctic frosty pastels & polar blue | `#88c0d0` / `#5e81ac` / `#81a1c1` / `#2e3440` |
| `forest` | Emerald moss & canopy shadows | `#55ff77` / `#2e8b57` / `#1b4332` / `#081c15` |
| `monochrome` | Crisp minimalist grayscale | `#ffffff` / `#999999` / `#444444` / `#0a0a0a` |
| `matrix` | Acid lime & phosphor terminal green | `#a6ff00` / `#00ff41` / `#003b00` / `#0d1117` |
| `sunset` | Royal plum & dusk orange | `#ff4500` / `#ff8c00` / `#4a0e4e` / `#1a0022` |
| `dracula` | Gothic vampire pink & cyan | `#ff79c6` / `#bd93f9` / `#8be9fd` / `#282a36` |
| `catppuccin` | Soothing Mocha pastels | `#f5c2e7` / `#cba6f7` / `#89b4fa` / `#1e1e2e` |
| `tokyo-night`| Neon metropolitan rain | `#f7768e` / `#bb9af7` / `#7aa2f7` / `#1a1b26` |

---

## 4. Usage & Examples

### CLI Command Examples

```bash
# Run with a curated preset
lavaterm --theme cyberpunk
lavaterm --theme synthwave
lavaterm --theme ocean
lavaterm --theme nord

# Automatically match desktop wallpaper colors (Pywal / Wallust)
lavaterm --theme auto

# Force Pywal scheme from cache
lavaterm --theme pywal

# Force Wallust scheme from cache
lavaterm --theme wallust

# Load a custom palette file (JSON or TOML)
lavaterm --theme ~/.config/lavaterm/my_theme.json
```

---

## 5. Configuration (TOML)

In `~/.config/lavaterm/config.toml`:

```toml
[theme]
# Select a preset, "auto", "pywal", "wallust", or a file path
name = "synthwave"

# Optional explicit path override
# path = "/home/user/.config/lavaterm/custom_theme.json"
```

---

## 6. Custom Theme File Format

You can write custom theme files in JSON or TOML:

### JSON Format (`my_theme.json`)
```json
{
  "bottom": "#ff0055",
  "middle": "#9900ff",
  "top": "#00f0ff",
  "background": "#0a0a14"
}
```

### TOML Format (`my_theme.toml`)
```toml
bottom = "#ff0055"
middle = "#9900ff"
top = "#00f0ff"
background = "#0a0a14"
```
