use glam::Vec2;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<u8>,
}

impl Map {
    pub fn from_rows(rows: &[&[u8]]) -> Self {
        Self {
            width: rows[0].len() as i32,
            height: rows.len() as i32,
            cells: rows.iter().flat_map(|row| row.iter().copied()).collect(),
        }
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return true;
        }
        self.cells[(y * self.width + x) as usize] != 0
    }

    pub fn is_wall_at(&self, position: Vec2) -> bool {
        self.is_wall(position.x.floor() as i32, position.y.floor() as i32)
    }
}
