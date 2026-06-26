use glam::Vec2;

pub struct Camera {
    pub fov: f32,
    pub view_direction: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 60.0,
            view_direction: Vec2::new(1.0, 0.0),
        }
    }
}
