# Themes & Color Palette Reference

ZenLavaTerm includes built-in presets, automatic desktop wallpaper theme detection, and support for external JSON and TOML color schemes.

---

## 1. Built-in Preset Palettes

Specify via `-t <PRESET>` or `theme = "<PRESET>"` in `config.toml`:

| Preset | Vibe & Dominant Colors |
|---|---|
| **`lava`** (Default) | Classic incandescent amber, crimson, and golden molten wax |
| **`ocean`** | Deep abyss navy, bioluminescent cyan, and aquamarine |
| **`cyberpunk`** | High-contrast neon pink, electric violet, and bright yellow |
| **`synthwave`** | Retro 80s dusk magenta, twilight purple, and solar sunset orange |
| **`monochrome`** | Minimalist grayscale charcoal, silver, and crisp white |
| **`toxic`** | Radioactive emerald, fluorescent lime, and noxious yellow |
| **`matrix`** | Dark terminal digital rain greens and pale phosphor highlights |
| **`dracula`** | Gothic purple, vibrant pink, and icy cyan |
| **`nord`** | Arctic polar ice blue, frost teal, and muted slate |
| **`gruvbox`** | Warm retro earthy ochre, rust orange, and olive amber |

---

## 2. Dynamic Desktop Theme Detection

Pass `--theme auto` to automatically synchronize ZenLavaTerm colors with your desktop environment:
1. **Pywal Integration**: Scans `$HOME/.cache/wal/colors.json` (or `%LOCALAPPDATA%\wal\colors.json` on Windows) for wallpaper-derived terminal palettes.
2. **Wallust Integration**: Scans `$HOME/.cache/wallust/colors.json` for fast wallust palette extractions.
3. **Graceful Fallback**: If desktop theme files are missing or malformed, ZenLavaTerm silently falls back to the default `lava` preset.

---

## 3. Custom Palette Files

You can pass a direct path to a custom JSON or TOML file via `--theme <PATH>`:

### JSON Format (`custom_theme.json`)
```json
{
  "name": "Custom Glow",
  "background": "#0d1117",
  "colors": [
    "#58a6ff",
    "#1f6feb",
    "#238636",
    "#2ea043",
    "#3fb950"
  ]
}
```

### TOML Format (`custom_theme.toml`)
```toml
name = "Sunset"
background = "#180e29"
color1 = "#7209b7"
color2 = "#b5179e"
color3 = "#f72585"
color4 = "#ff758f"
color5 = "#ffb3c1"
```
