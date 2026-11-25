use termcanvas::prelude::*;

use crate::render_target::{RenderTarget, Rgb};

pub struct TermcanvasTarget {
    canvas: Canvas,
}

impl TermcanvasTarget {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }

    pub fn end(self) {
        self.canvas.end();
    }
}

impl RenderTarget for TermcanvasTarget {
    fn width(&self) -> u32 {
        self.canvas.width()
    }

    fn height(&self) -> u32 {
        self.canvas.height()
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Rgb) {
        if y >= self.canvas.height() {
            return;
        }

        let flipped_y = self.canvas.height() - 1 - y;
        let [r, g, b] = color;

        self.canvas
            .set_pixel(x, flipped_y, mathi::rgb_to_u32(r, g, b));
    }

    fn present(&mut self) {
        self.canvas.render();
    }
}
