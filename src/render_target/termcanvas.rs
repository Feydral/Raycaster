use termcanvas::prelude::*;

use crate::render_target::{RenderTarget, Rgb};

const COLOR_LEVELS: u32 = 32;

fn quantize(channel: u8) -> u8 {
    let steps = COLOR_LEVELS - 1;
    ((channel as u32 * steps + 127) / 255 * 255 / steps) as u8
}

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

        let packed = mathi::rgb_to_u32(quantize(r), quantize(g), quantize(b));

        self.canvas.set_pixel(x, flipped_y, packed);
    }

    fn sync_size(&mut self) {
        let width = Canvas::terminal_width();
        let height = Canvas::terminal_height();

        if width != self.canvas.width() || height != self.canvas.height() {
            self.canvas.resize(width, height);
        }
    }

    fn present(&mut self) {
        self.canvas.render();
    }
}
