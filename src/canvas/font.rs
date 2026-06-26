use std::collections::HashMap;

pub struct Font {
    pub glyphs: HashMap<char, Vec<u8>>,
    glyph_height: u8,
}

impl Font {
    pub fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
            glyph_height: 0,
        }
    }

    pub fn load_from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path).expect("Failed to read font file");

        let mut font = Self::new();
        font.parse(&content);
        font
    }

    fn parse(&mut self, input: &str) {
        let mut lines = input.lines().peekable();

        let glyph_height = lines.next().expect("missing glyph height header");
        self.glyph_height = glyph_height
            .strip_prefix("glyph_height:")
            .expect("expected glyph height header")
            .trim()
            .parse()
            .expect("invalid glyph height");

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() || !line.starts_with('\'') {
                continue;
            }

            let ch = line.chars().nth(1).expect("missing glyph char");

            let mut bitmap = Vec::new();

            while let Some(l) = lines.next() {
                let l = l.trim();

                if l.starts_with(']') {
                    break;
                }

                for num in l.split(',') {
                    let num = num.trim();
                    if num.is_empty() {
                        continue;
                    }

                    bitmap.push(
                        num.parse::<u8>()
                            .expect(&format!("invalid number in glyph data of char '{}'", ch)),
                    );
                }
            }

            if bitmap.len() % self.glyph_height as usize != 0 {
                panic!("invalid glyph data of char '{}'", ch)
            }
            self.glyphs.insert(ch, bitmap);
        }
    }

    pub fn glyph_height(&self) -> u8 {
        self.glyph_height
    }
}
