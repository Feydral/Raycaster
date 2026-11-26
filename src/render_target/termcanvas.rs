use crossterm::event::KeyCode;
use termcanvas::prelude::*;

use crate::frontend::{FrameInput, Frontend};
use crate::player::PlayerInput;
use crate::render_target::{RenderTarget, Rgb};

const COLOR_LEVELS: u32 = 64;

fn quantize(color: Rgb) -> Rgb {
    let brightest = color[0].max(color[1]).max(color[2]);

    if brightest == 0 {
        return color;
    }

    let steps = (COLOR_LEVELS - 1) as f32;
    let level = (brightest as f32 / 255.0 * steps).round();
    let scale = level / steps * 255.0 / brightest as f32;

    color.map(|channel| (channel as f32 * scale).round().min(255.0) as u8)
}

pub struct TermcanvasTarget {
    canvas: Canvas,
    input: Input,
}

impl TermcanvasTarget {
    pub fn new() -> Self {
        let input = Input::new();
        let canvas = Canvas::new();

        Self { canvas, input }
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
        let [r, g, b] = quantize(color);

        let packed = mathi::rgb_to_u32(r, g, b);

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

fn axis(input: &Input, positive: char, negative: char) -> f32 {
    let mut value = 0.0;

    if input.is_key_pressed(KeyCode::Char(positive)) {
        value += 1.0;
    }

    if input.is_key_pressed(KeyCode::Char(negative)) {
        value -= 1.0;
    }

    value
}

impl Frontend for TermcanvasTarget {
    fn poll_input(&mut self) -> FrameInput {
        self.input.update().expect("Failed to update input");

        FrameInput {
            player: PlayerInput {
                forward: axis(&self.input, 'w', 's'),
                turn: axis(&self.input, 'd', 'a'),
                jump: self.input.is_key_down(KeyCode::Char(' ')),
            },
            quit: self.input.is_key_pressed(KeyCode::Esc),
        }
    }

    fn shutdown(self: Box<Self>) {
        let this = *self;
        this.canvas.end();
    }
}
