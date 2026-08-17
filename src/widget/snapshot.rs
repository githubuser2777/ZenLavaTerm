//! Single-shot ANSI True Color frame serializer for status bars and external scripts.

use crate::core::Simulation;
use crate::render::{
    rasterize_simulation_options, BrailleRenderer, ColorPalette, VirtualFramebuffer,
};
use crate::{LavaError, Result};
use std::io::Write;

/// Options configuring single-frame snapshot rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotOptions<'a> {
    pub cols: u16,
    pub rows: u16,
    pub renderer_type: &'a str,
    pub threshold: f32,
    pub warmup_steps: usize,
    pub gradient: bool,
}

impl<'a> Default for SnapshotOptions<'a> {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            renderer_type: "halfblock",
            threshold: 1.0,
            warmup_steps: 5,
            gradient: true,
        }
    }
}

/// Serializes a single frame of the simulation into a standalone ANSI True Color string with explicit gradient control.
#[allow(clippy::too_many_arguments)]
pub fn render_snapshot_options(
    sim: &mut Simulation,
    palette: &ColorPalette,
    cols: u16,
    rows: u16,
    renderer_type: &str,
    threshold: f32,
    warmup_steps: usize,
    gradient: bool,
) -> Result<String> {
    if cols == 0 || rows == 0 {
        return Err(LavaError::Render(
            "Snapshot dimensions must be non-zero".to_string(),
        ));
    }

    // 1. Advance simulation warmup steps to form natural lava contours
    let dt = 1.0 / 30.0;
    for _ in 0..warmup_steps {
        sim.step(dt);
    }

    // 2. Compute virtual framebuffer dimensions
    let (v_width, v_height) = match renderer_type {
        "block" => (cols as usize, rows as usize),
        "braille" => (cols as usize * 2, rows as usize * 4),
        _ => (cols as usize, rows as usize * 2),
    };

    let mut fb = VirtualFramebuffer::new(v_width, v_height, palette.background);
    rasterize_simulation_options(sim, &mut fb, palette, threshold, gradient);

    // 3. Serialize into in-memory ANSI string without absolute cursor positioning
    let mut out = Vec::with_capacity(cols as usize * rows as usize * 30);

    match renderer_type {
        "block" => {
            let mut last_fg = None;
            for row in 0..rows as usize {
                if row > 0 {
                    out.write_all(b"\n")?;
                }
                for col in 0..cols as usize {
                    let pixel = fb.get_pixel(col, row).unwrap_or_default();
                    if last_fg != Some(pixel) {
                        write!(out, "\x1b[38;2;{};{};{}m", pixel.r, pixel.g, pixel.b)?;
                        last_fg = Some(pixel);
                    }
                    out.write_all("█".as_bytes())?;
                }
            }
            write!(out, "\x1b[0m")?;
        }
        "braille" => {
            let mut last_fg = None;
            let bg_color = palette.background;
            for row in 0..rows as usize {
                if row > 0 {
                    out.write_all(b"\n")?;
                }
                for col in 0..cols as usize {
                    let (ch, fg) = BrailleRenderer::compute_cell(&fb, col, row, bg_color);
                    let current_fg = fg.unwrap_or(bg_color);
                    if last_fg != Some(current_fg) {
                        write!(
                            out,
                            "\x1b[38;2;{};{};{}m",
                            current_fg.r, current_fg.g, current_fg.b
                        )?;
                        last_fg = Some(current_fg);
                    }
                    let mut utf8_buf = [0u8; 4];
                    let encoded = ch.encode_utf8(&mut utf8_buf);
                    out.write_all(encoded.as_bytes())?;
                }
            }
            write!(out, "\x1b[0m")?;
        }
        _ => {
            // Halfblock (default)
            let mut last_fg = None;
            let mut last_bg = None;
            for row in 0..rows as usize {
                if row > 0 {
                    out.write_all(b"\n")?;
                }
                let top_y = row * 2;
                let btm_y = top_y + 1;
                for col in 0..cols as usize {
                    let top_col = fb.get_pixel(col, top_y).unwrap_or_default();
                    let btm_col = fb.get_pixel(col, btm_y).unwrap_or_default();

                    if last_fg != Some(top_col) {
                        write!(out, "\x1b[38;2;{};{};{}m", top_col.r, top_col.g, top_col.b)?;
                        last_fg = Some(top_col);
                    }
                    if last_bg != Some(btm_col) {
                        write!(out, "\x1b[48;2;{};{};{}m", btm_col.r, btm_col.g, btm_col.b)?;
                        last_bg = Some(btm_col);
                    }
                    out.write_all("▀".as_bytes())?;
                }
            }
            write!(out, "\x1b[0m")?;
        }
    }

    String::from_utf8(out).map_err(|e| LavaError::Render(e.to_string()))
}

/// Serializes a single frame of the simulation into a standalone ANSI True Color string with smooth gradients.
pub fn render_snapshot(
    sim: &mut Simulation,
    palette: &ColorPalette,
    cols: u16,
    rows: u16,
    renderer_type: &str,
    threshold: f32,
    warmup_steps: usize,
) -> Result<String> {
    render_snapshot_options(
        sim,
        palette,
        cols,
        rows,
        renderer_type,
        threshold,
        warmup_steps,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::PhysicsParams;

    fn create_test_sim() -> Simulation {
        let params = PhysicsParams {
            gravity: 0.12,
            buoyancy: 0.80,
            viscosity: 0.93,
            noise: 0.15,
            thermal_transfer_rate: 0.40,
        };
        Simulation::new(params, 6, 42)
    }

    #[test]
    fn test_snapshot_zero_dimensions_error() {
        let mut sim = create_test_sim();
        let palette = ColorPalette::default();
        assert!(render_snapshot(&mut sim, &palette, 0, 10, "halfblock", 1.0, 5).is_err());
        assert!(render_snapshot(&mut sim, &palette, 10, 0, "halfblock", 1.0, 5).is_err());
    }

    #[test]
    fn test_snapshot_micro_geometries_all_renderers() {
        let palette = ColorPalette::default();
        let micro_geometries = [(20, 1), (20, 2), (20, 3)];
        let renderers = ["halfblock", "block", "braille"];

        for &(cols, rows) in &micro_geometries {
            for renderer in &renderers {
                let mut sim = create_test_sim();
                let output = render_snapshot(&mut sim, &palette, cols, rows, renderer, 1.0, 3)
                    .expect("Snapshot generation succeeds");

                assert!(output.ends_with("\x1b[0m"), "Must end with ANSI reset");
                assert!(
                    output.contains("\x1b[38;2;"),
                    "Must contain True Color SGR sequences"
                );

                if rows > 1 {
                    let newline_count = output.matches('\n').count();
                    assert_eq!(
                        newline_count,
                        (rows - 1) as usize,
                        "Must contain exact row newline separators"
                    );
                } else {
                    assert!(
                        !output.contains('\n'),
                        "Single-row snapshot must not contain newline"
                    );
                }
            }
        }
    }

    #[test]
    fn test_snapshot_standard_geometries() {
        let palette = ColorPalette::default();
        let mut sim = create_test_sim();
        let output = render_snapshot(&mut sim, &palette, 24, 8, "halfblock", 1.0, 5)
            .expect("Standard snapshot succeeds");

        assert!(output.contains('▀'));
        assert_eq!(output.matches('\n').count(), 7);
    }

    #[test]
    fn test_snapshot_gradient_option() {
        let palette = ColorPalette::default();
        let mut sim1 = create_test_sim();
        let mut sim2 = create_test_sim();

        let out_grad =
            render_snapshot_options(&mut sim1, &palette, 20, 6, "halfblock", 1.0, 3, true)
                .expect("Snapshot with gradient succeeds");
        let out_flat =
            render_snapshot_options(&mut sim2, &palette, 20, 6, "halfblock", 1.0, 3, false)
                .expect("Snapshot without gradient succeeds");

        assert!(out_grad.contains('▀'));
        assert!(out_flat.contains('▀'));
    }
}
