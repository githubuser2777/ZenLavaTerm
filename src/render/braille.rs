//! High-resolution Braille dot-matrix (2x4 sub-pixel) terminal renderer.

use super::color::Rgb;
use super::framebuffer::VirtualFramebuffer;
use super::Renderer;
use std::io::{self, Write};

/// Terminal renderer that packs 2x4 virtual sub-pixels into a single Unicode Braille character cell (`U+2800`..`U+28FF`).
#[derive(Debug, Default)]
pub struct BrailleRenderer {
    last_fg: Option<Rgb>,
    last_bg: Option<Rgb>,
}

impl BrailleRenderer {
    /// Creates a new `BrailleRenderer`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes the 8-bit Braille offset bitmask and average foreground color from an 8-pixel matrix.
    ///
    /// Braille standard dot mapping:
    /// - Dot 1: (0,0) -> 0x01
    /// - Dot 2: (0,1) -> 0x02
    /// - Dot 3: (0,2) -> 0x04
    /// - Dot 4: (1,0) -> 0x08
    /// - Dot 5: (1,1) -> 0x10
    /// - Dot 6: (1,2) -> 0x20
    /// - Dot 7: (0,3) -> 0x40
    /// - Dot 8: (1,3) -> 0x80
    pub fn compute_cell(
        buffer: &VirtualFramebuffer,
        cell_x: usize,
        cell_y: usize,
        bg: Rgb,
    ) -> (char, Option<Rgb>) {
        let base_x = cell_x * 2;
        let base_y = cell_y * 4;
        let width = buffer.width;
        let height = buffer.height;
        let slice = buffer.as_slice();

        let dot_offsets = [
            (0, 0, 0x01), // Dot 1
            (0, 1, 0x02), // Dot 2
            (0, 2, 0x04), // Dot 3
            (1, 0, 0x08), // Dot 4
            (1, 1, 0x10), // Dot 5
            (1, 2, 0x20), // Dot 6
            (0, 3, 0x40), // Dot 7
            (1, 3, 0x80), // Dot 8
        ];

        let mut mask: u32 = 0;
        let mut r_sum: u32 = 0;
        let mut g_sum: u32 = 0;
        let mut b_sum: u32 = 0;
        let mut active_dots: u32 = 0;

        for &(dx, dy, bit) in &dot_offsets {
            let px = base_x + dx;
            let py = base_y + dy;
            if px < width && py < height {
                let idx = py * width + px;
                let col = slice[idx];
                if col != bg {
                    mask |= bit;
                    r_sum += col.r as u32;
                    g_sum += col.g as u32;
                    b_sum += col.b as u32;
                    active_dots += 1;
                }
            }
        }

        let ch = char::from_u32(0x2800 + mask).unwrap_or(' ');
        let fg = std::num::NonZeroU32::new(active_dots).map(|divisor| {
            let d = divisor.get();
            Rgb::new((r_sum / d) as u8, (g_sum / d) as u8, (b_sum / d) as u8)
        });

        (ch, fg)
    }
}

impl Renderer for BrailleRenderer {
    fn render(&mut self, buffer: &VirtualFramebuffer, writer: &mut dyn Write) -> io::Result<()> {
        let term_rows = buffer.height / 4;
        let term_cols = buffer.width / 2;

        self.last_fg = None;
        self.last_bg = None;

        // Background color assumed from top-left or default dark
        let bg_color = buffer.get_pixel(0, 0).unwrap_or_default();

        for row in 0..term_rows {
            write!(writer, "\x1b[{};1H", row + 1)?;

            for col in 0..term_cols {
                let (ch, fg) = Self::compute_cell(buffer, col, row, bg_color);
                let current_fg = fg.unwrap_or(bg_color);

                if self.last_fg != Some(current_fg) {
                    write!(
                        writer,
                        "\x1b[38;2;{};{};{}m",
                        current_fg.r, current_fg.g, current_fg.b
                    )?;
                    self.last_fg = Some(current_fg);
                }

                if self.last_bg != Some(bg_color) {
                    write!(
                        writer,
                        "\x1b[48;2;{};{};{}m",
                        bg_color.r, bg_color.g, bg_color.b
                    )?;
                    self.last_bg = Some(bg_color);
                }

                let mut utf8_buf = [0u8; 4];
                let encoded = ch.encode_utf8(&mut utf8_buf);
                writer.write_all(encoded.as_bytes())?;
            }
        }

        write!(writer, "\x1b[0m")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_empty_cell() {
        let bg = Rgb::new(10, 10, 10);
        let fb = VirtualFramebuffer::new(2, 4, bg);
        let (ch, fg) = BrailleRenderer::compute_cell(&fb, 0, 0, bg);
        assert_eq!(ch, '\u{2800}'); // Empty braille pattern
        assert_eq!(fg, None);
    }

    #[test]
    fn test_braille_single_dot_and_all_dots() {
        let bg = Rgb::new(0, 0, 0);
        let mut fb = VirtualFramebuffer::new(2, 4, bg);

        // Turn on dot 1 (0, 0)
        let red = Rgb::new(255, 0, 0);
        fb.set_pixel(0, 0, red);
        let (ch1, fg1) = BrailleRenderer::compute_cell(&fb, 0, 0, bg);
        assert_eq!(ch1, '\u{2801}'); // Dot 1
        assert_eq!(fg1, Some(red));

        // Turn on all 8 dots
        for y in 0..4 {
            for x in 0..2 {
                fb.set_pixel(x, y, red);
            }
        }
        let (ch_all, fg_all) = BrailleRenderer::compute_cell(&fb, 0, 0, bg);
        assert_eq!(ch_all, '\u{28FF}'); // Full 8-dot braille character
        assert_eq!(fg_all, Some(red));
    }

    #[test]
    fn test_braille_renderer_output() {
        let mut renderer = BrailleRenderer::new();
        let bg = Rgb::new(0, 0, 0);
        let mut fb = VirtualFramebuffer::new(2, 4, bg);
        fb.set_pixel(1, 3, Rgb::new(0, 255, 0)); // Dot 8 -> 0x80

        let mut output = Vec::new();
        renderer.render(&fb, &mut output).expect("Render succeeds");

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains('\u{2880}'));
        assert!(text.contains("\x1b[0m"));
    }
}
