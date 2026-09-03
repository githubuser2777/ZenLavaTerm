//! In-memory 2D virtual pixel canvas and double-buffering support.

use super::color::Rgb;

/// A 2D grid of 24-bit RGB pixels representing offscreen virtual canvas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualFramebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Rgb>,
}

impl VirtualFramebuffer {
    /// Creates a new virtual framebuffer with dimensions `width x height` filled with `fill`.
    pub fn new(width: usize, height: usize, fill: Rgb) -> Self {
        let size = width.saturating_mul(height);
        Self {
            width,
            height,
            pixels: vec![fill; size],
        }
    }

    /// Resizes the framebuffer, preserving existing pixels when possible or filling new areas with `fill`.
    pub fn resize(&mut self, new_width: usize, new_height: usize, fill: Rgb) {
        if self.width == new_width && self.height == new_height {
            return;
        }

        let mut new_pixels = vec![fill; new_width.saturating_mul(new_height)];
        let copy_w = self.width.min(new_width);
        let copy_h = self.height.min(new_height);

        for y in 0..copy_h {
            for x in 0..copy_w {
                let old_idx = y * self.width + x;
                let new_idx = y * new_width + x;
                if old_idx < self.pixels.len() && new_idx < new_pixels.len() {
                    new_pixels[new_idx] = self.pixels[old_idx];
                }
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.pixels = new_pixels;
    }

    /// Clears the entire buffer with a solid fill color.
    pub fn clear(&mut self, fill: Rgb) {
        self.pixels.fill(fill);
    }

    /// Sets the pixel at `(x, y)`. Safely ignores out-of-bounds coordinates.
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            if idx < self.pixels.len() {
                self.pixels[idx] = color;
            }
        }
    }

    /// Retrieves the pixel color at `(x, y)`.
    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Rgb> {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.pixels.get(idx).copied()
        } else {
            None
        }
    }

    /// Returns a direct slice to the contiguous pixel array.
    #[inline]
    pub fn as_slice(&self) -> &[Rgb] {
        &self.pixels
    }

    /// Returns a mutable direct slice to the contiguous pixel array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Rgb] {
        &mut self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_bounds_and_pixels() {
        let mut fb = VirtualFramebuffer::new(10, 10, Rgb::new(0, 0, 0));
        assert_eq!(fb.get_pixel(5, 5), Some(Rgb::new(0, 0, 0)));

        fb.set_pixel(5, 5, Rgb::new(255, 100, 50));
        assert_eq!(fb.get_pixel(5, 5), Some(Rgb::new(255, 100, 50)));

        // Out of bounds
        fb.set_pixel(15, 15, Rgb::new(255, 255, 255));
        assert_eq!(fb.get_pixel(15, 15), None);
    }

    #[test]
    fn test_framebuffer_resize() {
        let mut fb = VirtualFramebuffer::new(4, 4, Rgb::new(10, 10, 10));
        fb.set_pixel(1, 1, Rgb::new(99, 99, 99));

        fb.resize(8, 8, Rgb::new(20, 20, 20));
        assert_eq!(fb.width, 8);
        assert_eq!(fb.height, 8);
        assert_eq!(fb.get_pixel(1, 1), Some(Rgb::new(99, 99, 99)));
        assert_eq!(fb.get_pixel(6, 6), Some(Rgb::new(20, 20, 20)));
    }
}
