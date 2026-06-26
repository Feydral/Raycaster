pub mod draw;
pub mod font;

use crossterm::terminal;
use std::io::{Write, stdout};

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    out: Vec<u8>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        let (w, h_half) = terminal::size().unwrap();
        let h = h_half * 2;
        let size = w as usize * h as usize;
        print!("\x1b[3J\x1b[H\x1b[?25l\x1b[?1049h");
        stdout().flush().unwrap();
        Self {
            width: w as u32,
            height: h as u32,
            pixels: vec![0; size],
            out: Vec::with_capacity(w as usize * h_half as usize * 25),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[y as usize * self.width as usize + x as usize] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn render(&mut self) {
        self.out.clear();
        self.out.extend_from_slice(b"\x1b[?2026h\x1b[H");

        let rows = self.height / 2;
        let mut last_fg = 0;
        let mut last_bg = 0;

        self.out
            .extend_from_slice(b"\x1b[38;2;0;0;0m\x1b[48;2;0;0;0m");

        for row in 0..rows {
            let inv = rows - 1 - row;
            let y_top = inv * 2 + 1;
            let y_bottom = inv * 2;

            for x in 0..self.width {
                let fg = self.get_pixel(x, y_top);
                let bg = self.get_pixel(x, y_bottom);

                if fg != last_fg {
                    write!(
                        &mut self.out,
                        "\x1b[38;2;{};{};{}m",
                        (fg >> 24) as u8,
                        (fg >> 16) as u8,
                        (fg >> 8) as u8
                    )
                    .unwrap();
                    last_fg = fg;
                }
                if bg != last_bg {
                    write!(
                        &mut self.out,
                        "\x1b[48;2;{};{};{}m",
                        (bg >> 24) as u8,
                        (bg >> 16) as u8,
                        (bg >> 8) as u8
                    )
                    .unwrap();
                    last_bg = bg;
                }

                self.out.extend_from_slice("▀".as_bytes());
            }
        }

        self.out.extend_from_slice(b"\x1b[0m\x1b[?2026l");
        let mut stdout = stdout();
        stdout.write_all(&self.out).unwrap();
        stdout.flush().unwrap();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        self.pixels.clear();
        self.pixels.resize((new_width * new_height) as usize, 0);
        self.out.clear();
        self.out
            .reserve(new_width as usize * (new_height / 2) as usize * 20);
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn end(self) {
        print!("\x1b[?25h\x1b[?1049l\x1b[2J\x1b[3J");
        stdout().flush().unwrap();
    }

    pub fn terminal_width() -> u32 {
        terminal::size().expect("terminal::size()").0 as u32
    }
    pub fn terminal_height() -> u32 {
        2 * terminal::size().expect("terminal::size()").1 as u32
    }
}
