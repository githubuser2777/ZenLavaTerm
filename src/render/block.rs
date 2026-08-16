//! Full-block (`█`) fallback terminal renderer.

use super::color::Rgb;
use super::framebuffer::VirtualFramebuffer;
use super::Renderer;
use std::io::{self, Write};

/// Terminal renderer using full block characters (`█`).
#[derive(Debug, Default)]
pub struct BlockRenderer {
    last_fg: Option<Rgb>,
}

impl BlockRenderer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for BlockRenderer {
    fn render(&mut self, buffer: &VirtualFramebuffer, writer: &mut dyn Write) -> io::Result<()> {
        let term_rows = buffer.height;
        let term_cols = buffer.width;

        self.last_fg = None;

        for row in 0..term_rows {
            write!(writer, "\x1b[{};1H", row + 1)?;

            for col in 0..term_cols {
                let color = buffer.get_pixel(col, row).unwrap_or_default();

                if self.last_fg != Some(color) {
                    write!(writer, "\x1b[38;2;{};{};{}m", color.r, color.g, color.b)?;
                    self.last_fg = Some(color);
                }

                writer.write_all("█".as_bytes())?;
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
    fn test_block_render_output() {
        let mut renderer = BlockRenderer::new();
        let fb = VirtualFramebuffer::new(2, 2, Rgb::new(0, 0, 255));

        let mut output = Vec::new();
        renderer.render(&fb, &mut output).expect("Render succeeds");

        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("█"));
        assert!(text.contains("\x1b[0m"));
    }
}
