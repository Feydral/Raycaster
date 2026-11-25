use glam::Vec2;

use crate::camera::Camera;
use crate::map::Map;

pub struct PlayerInput {
    pub forward: f32,
    pub strafe: f32,
    pub turn: f32,
}

pub struct Player {
    pub camera: Camera,
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            position: Vec2::ONE,
            velocity: Vec2::ZERO,
            rotation_speed: 2.0,
            acceleration: 50.0,
            friction: 12.0,
        }
    }

    pub fn update(&mut self, input: &PlayerInput, dt: f32, map: &Map) {
        let angle = input.turn * self.rotation_speed * dt;
        self.camera.view_direction = Vec2::from_angle(angle)
            .rotate(self.camera.view_direction)
            .normalize();

        let view_direction = self.camera.view_direction;
        let right = Vec2::new(-view_direction.y, view_direction.x);

        let move_input = view_direction * input.forward + right * input.strafe;

        if move_input.length() > 0.0 {
            self.velocity += move_input.normalize() * self.acceleration * dt;
        }

        self.velocity *= (1.0 - self.friction * dt).max(0.0);

        let movement = self.velocity * dt;

        let new_x = self.position.x + movement.x;
        let new_y = self.position.y + movement.y;

        if !map.is_wall_at(Vec2::new(new_x, self.position.y)) {
            self.position.x = new_x;
        } else {
            self.velocity.x = 0.0;
        }

        if !map.is_wall_at(Vec2::new(self.position.x, new_y)) {
            self.position.y = new_y;
        } else {
            self.velocity.y = 0.0;
        }
    }
}
