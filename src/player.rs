use glam::Vec2;

use crate::camera::Camera;
use crate::map::Map;

pub struct PlayerInput {
    pub forward: f32,
    pub turn: f32,
    pub jump: bool,
}

pub struct Player {
    pub camera: Camera,
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub jump_offset: f32,
    pub jump_velocity: f32,
    pub jump_strength: f32,
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
            radius: 0.25,
            jump_offset: 0.0,
            jump_velocity: 0.0,
            jump_strength: 2.5,
            rotation_speed: 2.0,
            acceleration: 25.0,
            friction: 12.0,
        }
    }

    pub fn update(&mut self, input: &PlayerInput, dt: f32, map: &Map) {
        let angle = input.turn * self.rotation_speed * dt;
        self.camera.view_direction = Vec2::from_angle(angle)
            .rotate(self.camera.view_direction)
            .normalize();

        let view_direction = self.camera.view_direction;

        if input.forward != 0.0 {
            self.velocity += view_direction * input.forward * self.acceleration * dt;
        }

        self.velocity *= (1.0 - self.friction * dt).max(0.0);

        if input.jump && self.jump_offset == 0.0 {
            self.jump_velocity = self.jump_strength;

            if self.velocity.length() > 0.01 {
                self.velocity += view_direction * 0.9;
            }
        }

        self.jump_velocity -= 9.81 * dt;
        self.jump_offset += self.jump_velocity * dt;

        if self.jump_offset <= 0.0 {
            self.jump_offset = 0.0;
            self.jump_velocity = 0.0;
        }

        let movement = self.velocity * dt;

        let new_x = self.position.x + movement.x;
        let new_y = self.position.y + movement.y;

        if !map.overlaps_wall(Vec2::new(new_x, self.position.y), self.radius) {
            self.position.x = new_x;
        } else {
            self.velocity.x = 0.0;
        }

        if !map.overlaps_wall(Vec2::new(self.position.x, new_y), self.radius) {
            self.position.y = new_y;
        } else {
            self.velocity.y = 0.0;
        }
    }
}
