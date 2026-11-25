pub struct Map {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<u8>,
}

impl Map {
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return true;
        }
        self.cells[(y * self.width + x) as usize] != 0
    }
}
