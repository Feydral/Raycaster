use nanofb::prelude::*;

use crate::frontend::{FrameInput, Frontend};
use crate::player::PlayerInput;
use crate::render_target::{RenderTarget, Rgb};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const BUFFER_WIDTH: u32 = 320;
const BUFFER_HEIGHT: u32 = 180;

pub struct NanofbTarget {
    window: Window,
    buffer: Vec<Color>,
}

impl NanofbTarget {
    pub fn new() -> Self {
        let options = WindowOptions {
            title: "raycaster".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            buffer_width: BUFFER_WIDTH,
            buffer_height: BUFFER_HEIGHT,
            aspect_mode: AspectMode::AspectFit,
            ..Default::default()
        };

        let window = Window::new(options).expect("Failed to create window");
        let buffer = vec![Color::BLACK; (BUFFER_WIDTH * BUFFER_HEIGHT) as usize];

        Self { window, buffer }
    }
}

impl RenderTarget for NanofbTarget {
    fn width(&self) -> u32 {
        self.window.buffer_width()
    }

    fn height(&self) -> u32 {
        self.window.buffer_height()
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Rgb) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let [r, g, b] = color;
        let index = (y * self.width() + x) as usize;

        self.buffer[index] = Color::from_rgb(r, g, b);
    }

    fn sync_size(&mut self) {}

    fn present(&mut self) {
        self.window
            .present(&self.buffer)
            .expect("Failed to present buffer");
    }
}

fn axis(input: &InputSnapshot, positive: Key, negative: Key) -> f32 {
    let mut value = 0.0;

    if input.is_key_held(positive) {
        value += 1.0;
    }

    if input.is_key_held(negative) {
        value -= 1.0;
    }

    value
}

impl Frontend for NanofbTarget {
    fn poll_input(&mut self) -> FrameInput {
        for event in self.window.poll_events() {
            if let Event::CloseRequested = event {
                self.window.close();
            }
        }

        let input = self.window.input_snapshot();

        FrameInput {
            player: PlayerInput {
                forward: axis(&input, Key::W, Key::S),
                turn: axis(&input, Key::D, Key::A),
                jump: input.is_key_pressed(Key::Space),
            },
            quit: self.window.should_close() || input.is_key_pressed(Key::Escape),
        }
    }
}
