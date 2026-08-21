//! Terminal rendering, color interpolation, and framebuffer abstractions.

pub mod block;
pub mod braille;
pub mod color;
pub mod framebuffer;
pub mod halfblock;

pub use block::BlockRenderer;
pub use braille::BrailleRenderer;
pub use color::{ColorPalette, Rgb};
pub use framebuffer::VirtualFramebuffer;
pub use halfblock::HalfBlockRenderer;

use crate::core::Simulation;
use std::io::Write;

/// Common interface implemented by terminal renderers.
pub trait Renderer {
    /// Renders the contents of the virtual framebuffer to the provided output writer.
    fn render(
        &mut self,
        buffer: &VirtualFramebuffer,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
}

/// Rasterizes the continuous simulation into the discrete virtual framebuffer using the given palette, threshold, and gradient mode.
pub fn rasterize_simulation_options(
    sim: &Simulation,
    buffer: &mut VirtualFramebuffer,
    palette: &ColorPalette,
    threshold: f32,
    gradient: bool,
) {
    let width = buffer.width;
    let height = buffer.height;

    if width == 0 || height == 0 {
        return;
    }

    let inv_w = 1.0 / width as f32;
    let inv_h = 1.0 / height as f32;
    let pixels = buffer.as_mut_slice();

    for y in 0..height {
        // Map discrete y [0..height] to normalized space [0.0..1.0] (y=0 top in terminal, so invert for simulation bottom heat)
        let sim_y = 1.0 - (y as f32 + 0.5) * inv_h;
        let row_start = y * width;

        for x in 0..width {
            let sim_x = (x as f32 + 0.5) * inv_w;

            let (field_val, temp) = sim.evaluate_at(sim_x, sim_y);
            let pixel_color = palette.sample_lava_stepped(temp, field_val, threshold, gradient);

            pixels[row_start + x] = pixel_color;
        }
    }
}

/// Rasterizes the continuous simulation into the discrete virtual framebuffer using smooth gradient interpolation.
pub fn rasterize_simulation(
    sim: &Simulation,
    buffer: &mut VirtualFramebuffer,
    palette: &ColorPalette,
    threshold: f32,
) {
    rasterize_simulation_options(sim, buffer, palette, threshold, true);
}
