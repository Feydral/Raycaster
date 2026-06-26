pub struct Map {
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new() -> Self {
        Self { grid: vec![] }
    }

    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        let margin = 0.25;
        let corners = [
            (x - margin, y - margin),
            (x + margin, y - margin),
            (x - margin, y + margin),
            (x + margin, y + margin),
        ];
        corners.iter().any(|&(cx, cy)| {
            if cx < 0.0 || cy < 0.0 {
                return true;
            }
            let gx = cx as usize;
            let gy = cy as usize;
            if gx >= self.grid[0].len() || gy >= self.grid.len() {
                return true;
            }
            self.grid[gy][gx] != 0
        })
    }

    pub fn is_wall_cell(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        let gx = x as usize;
        let gy = y as usize;
        if gx >= self.grid[0].len() || gy >= self.grid.len() {
            return true;
        }
        self.grid[gy][gx] != 0
    }
}
