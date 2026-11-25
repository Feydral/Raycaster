use glam::Vec2;

pub struct Camera {
    pub view_direction: Vec2,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view_direction: Vec2::new(1.0, 0.0),
            fov: 80f32.to_radians(),
        }
    }
}
