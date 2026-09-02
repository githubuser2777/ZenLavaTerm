//! High-resolution Half-Block (`▀` / `▄`) terminal renderer.

use super::color::Rgb;
use super::framebuffer::VirtualFramebuffer;
use super::Renderer;
use std::io::{self, Write};

/// Terminal renderer that packs two vertical virtual pixels into a single character cell using `▀`.
#[derive(Debug, Default)]
pub struct HalfBlockRenderer {
    last_fg: Option<Rgb>,
    last_bg: Option<Rgb>,
}

impl HalfBlockRenderer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for HalfBlockRenderer {
    fn render(&mut self, buffer: &VirtualFramebuffer, writer: &mut dyn Write) -> io::Result<()> {
        let term_rows = buffer.height / 2;
        let term_cols = buffer.width;
        let slice = buffer.as_slice();

        self.last_fg = None;
        self.last_bg = None;

        for row in 0..term_rows {
            // ponytail: direct formatted cell writes; line-buffered/diff-based damage grid if terminal IO saturates
            let top_row_start = (row * 2) * term_cols;
            let btm_row_start = (row * 2 + 1) * term_cols;

            // Move cursor to row (1-indexed in ANSI)
            write!(writer, "\x1b[{};1H", row + 1)?;

            for col in 0..term_cols {
                let top_col = slice[top_row_start + col];
                let btm_col = slice[btm_row_start + col];

                if self.last_fg != Some(top_col) {
                    write!(
                        writer,
                        "\x1b[38;2;{};{};{}m",
                        top_col.r, top_col.g, top_col.b
                    )?;
                    self.last_fg = Some(top_col);
                }

                if self.last_bg != Some(btm_col) {
                    write!(
                        writer,
                        "\x1b[48;2;{};{};{}m",
                        btm_col.r, btm_col.g, btm_col.b
                    )?;
                    self.last_bg = Some(btm_col);
                }

                writer.write_all("▀".as_bytes())?;
            }
        }

        // Reset styling at the end of the frame
        write!(writer, "\x1b[0m")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfblock_render_output() {
        let mut renderer = HalfBlockRenderer::new();
        let mut fb = VirtualFramebuffer::new(2, 2, Rgb::new(255, 0, 0));
        fb.set_pixel(0, 1, Rgb::new(0, 255, 0));

        let mut output = Vec::new();
        renderer.render(&fb, &mut output).expect("Render succeeds");

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("▀"));
        assert!(text.contains("\x1b[0m"));
    }
}
