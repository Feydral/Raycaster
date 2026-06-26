use crate::canvas::{Canvas, font::Font};

pub struct DrawBuilder<'a> {
    canvas: &'a mut Canvas,
    font: &'a Font,

    x: u32,
    y: u32,
    color: u32,
    align: Align,
}

#[allow(dead_code)]
impl<'a> DrawBuilder<'a> {
    pub fn at(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn x(mut self, x: u32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u32) -> Self {
        self.y = y;
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn text(self, text: impl Into<String>) {
        self.canvas.draw_text(
            &text.into(),
            self.x,
            self.y,
            self.color,
            self.align,
            self.font,
        );
    }

    pub fn uint(self, value: u32) {
        self.canvas
            .draw_uint(value, self.x, self.y, self.color, self.align, self.font);
    }

    pub fn int(self, value: i32, always_sign: bool) {
        self.canvas.draw_int(
            value,
            self.x,
            self.y,
            self.color,
            self.align,
            self.font,
            always_sign,
        );
    }

    pub fn float(self, value: f32, decimals: usize, always_sign: bool) {
        self.canvas.draw_float(
            value,
            self.x,
            self.y,
            self.color,
            self.align,
            self.font,
            decimals,
            always_sign,
        );
    }
}

pub enum Align {
    Left,
    Right,
}

impl Canvas {
    pub fn draw<'a>(&'a mut self, font: &'a Font) -> DrawBuilder<'a> {
        DrawBuilder {
            canvas: self,
            font,
            x: 0,
            y: 0,
            color: 0xFFFFFFFF,
            align: Align::Left,
        }
    }

    fn draw_text(&mut self, text: &str, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let glyph_height = font.glyph_height() as u32;
        let tab_size = 4;

        let mut cursor_y = y;

        for line in text.split('\n') {
            let mut line_width = 0i32;
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                match c {
                    '\t' => {
                        let tab_w = (glyph_height * tab_size) as i32;
                        line_width += tab_w;
                    }
                    '\\' => match chars.peek() {
                        Some('n') | Some('t') | Some('\\') => {
                            let c2 = chars.next().unwrap();
                            let actual = match c2 {
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',
                                _ => c2,
                            };

                            if let Some(g) = font.glyphs.get(&actual) {
                                line_width += g.len() as i32 / glyph_height as i32;
                            }
                        }
                        _ => {
                            if let Some(g) = font.glyphs.get(&'\\') {
                                line_width += g.len() as i32 / glyph_height as i32;
                            }
                        }
                    },
                    _ => {
                        if let Some(g) = font.glyphs.get(&c) {
                            line_width += g.len() as i32 / glyph_height as i32;
                        }
                    }
                }
            }

            let mut cursor_x = match align {
                Align::Left => x as i32,
                Align::Right => x as i32 - line_width,
            };

            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                match c {
                    '\t' => {
                        let tab_w = (glyph_height * tab_size) as i32;
                        let offset = cursor_x - x as i32;
                        cursor_x = x as i32 + ((offset / tab_w) + 1) * tab_w;
                    }

                    '\\' => {
                        let draw_char = match chars.peek() {
                            Some('n') => {
                                chars.next();
                                '\n'
                            }
                            Some('t') => {
                                chars.next();
                                '\t'
                            }
                            Some('\\') => {
                                chars.next();
                                '\\'
                            }
                            _ => '\\',
                        };

                        if draw_char == '\n' {
                            break;
                        }

                        if draw_char == '\t' {
                            let tab_w = (glyph_height * tab_size) as i32;
                            let offset = cursor_x - x as i32;
                            cursor_x = x as i32 + ((offset / tab_w) + 1) * tab_w;
                            continue;
                        }

                        if let Some(g) = font.glyphs.get(&draw_char) {
                            let w = g.len() as i32 / glyph_height as i32;
                            self.draw_character(
                                draw_char,
                                cursor_x as u32,
                                cursor_y,
                                color,
                                Align::Left,
                                font,
                            );
                            cursor_x += w;
                        }
                    }

                    _ => {
                        if let Some(g) = font.glyphs.get(&c) {
                            let w = g.len() as i32 / glyph_height as i32;
                            self.draw_character(
                                c,
                                cursor_x as u32,
                                cursor_y,
                                color,
                                Align::Left,
                                font,
                            );
                            cursor_x += w;
                        }
                    }
                }
            }

            cursor_y += glyph_height + 2;
        }
    }

    fn draw_character(&mut self, c: char, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let Some(glyph) = font.glyphs.get(&c) else {
            return;
        };

        let height = font.glyph_height() as i32;
        let width = glyph.len() as i32 / height;

        let x = x as i32;
        let y = y as i32;

        let start_x = match align {
            Align::Left => x,
            Align::Right => x - width,
        };

        for gy in 0..height {
            for gx in 0..width {
                let idx = (gy * width + gx) as usize;

                if glyph[idx] == 1 {
                    let dx = start_x + gx;
                    let dy = y + gy;

                    if dx < 0 || dy < 0 || dx >= self.width as i32 || dy >= self.height as i32 {
                        continue;
                    }

                    let draw_y = self.height as i32 - 1 - dy;
                    self.set_pixel(dx as u32, draw_y as u32, color);
                }
            }
        }
    }

    fn draw_uint(&mut self, value: u32, x: u32, y: u32, color: u32, align: Align, font: &Font) {
        let glyph_height = font.glyph_height() as i32;
        let digits: Vec<char> = value.to_string().chars().collect();

        let mut total_width = 0i32;
        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left => x as i32,
            Align::Right => x as i32 - total_width,
        };

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }

    fn draw_int(
        &mut self,
        value: i32,
        x: u32,
        y: u32,
        color: u32,
        align: Align,
        font: &Font,
        always_show_sign: bool,
    ) {
        let glyph_height = font.glyph_height() as i32;

        let (sign_char, abs_val): (char, u32) = if value < 0 {
            ('-', value.wrapping_abs() as u32)
        } else {
            ('+', value as u32)
        };

        let show_sign = value < 0 || always_show_sign;
        let digits: Vec<char> = abs_val.to_string().chars().collect();

        let mut total_width = 0i32;

        if show_sign {
            if let Some(glyph) = font.glyphs.get(&sign_char) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left => x as i32,
            Align::Right => x as i32 - total_width,
        };

        if show_sign {
            if let Some(glyph) = font.glyphs.get(&sign_char) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(sign_char, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }

        for &c in &digits {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }

    fn draw_float(
        &mut self,
        value: f32,
        x: u32,
        y: u32,
        color: u32,
        align: Align,
        font: &Font,
        decimals: usize,
        always_show_sign: bool,
    ) {
        let glyph_height = font.glyph_height() as i32;

        let mut s = format!("{:+.*}", decimals + 5, value);

        if let Some(dot) = s.find('.') {
            let end = dot + 1 + decimals;
            if s.len() > end {
                s.truncate(end);
            }
        }

        if !always_show_sign && s.starts_with('+') {
            s.remove(0);
        }

        let chars: Vec<char> = s.chars().collect();

        let mut total_width = 0i32;
        for &c in &chars {
            if let Some(glyph) = font.glyphs.get(&c) {
                total_width += glyph.len() as i32 / glyph_height;
            }
        }

        let mut cursor_x = match align {
            Align::Left => x as i32,
            Align::Right => x as i32 - total_width,
        };

        for &c in &chars {
            if let Some(glyph) = font.glyphs.get(&c) {
                let w = glyph.len() as i32 / glyph_height;
                self.draw_character(c, cursor_x as u32, y, color, Align::Left, font);
                cursor_x += w;
            }
        }
    }
}
