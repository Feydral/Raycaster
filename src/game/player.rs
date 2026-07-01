use crossterm::event::KeyCode;
use glam::Vec2;
use termcanvas::prelude::*;

use crate::{game::camera::Camera, raycaster::map::Map};

pub struct Player {
    pub camera: Camera,
    pub position: Vec2,

    pub rotation_speed: f32,

    pub jump_strength: f32,
    pub jump_offset: f32,
    pub jump_velocity: f32,

    pub velocity: Vec2,
    pub acceleration: f32,
    pub friction: f32,
}

impl Player {
    pub fn on_start() -> Self {
        let mut camera = Camera::new();
        camera.fov = 80.0;

        Self {
            camera,
            position: Vec2::ONE,

            rotation_speed: 2.0,

            jump_strength: 2.5,
            jump_offset: 0.0,
            jump_velocity: 0.0,

            velocity: Vec2::ZERO,
            acceleration: 50.0,
            friction: 12.0,
        }
    }

    pub fn on_update(&mut self, input: &Input, dt: f32, map: &Map) {
        if input.is_key_pressed(KeyCode::Char('a')) {
            let cos_a = (-self.rotation_speed * dt).cos();
            let sin_a = (-self.rotation_speed * dt).sin();

            let new_x = self.camera.view_direction.x * cos_a - self.camera.view_direction.y * sin_a;
            let new_y = self.camera.view_direction.x * sin_a + self.camera.view_direction.y * cos_a;

            self.camera.view_direction = Vec2::new(new_x, new_y).normalize();
        }

        if input.is_key_pressed(KeyCode::Char('d')) {
            let cos_a = (self.rotation_speed * dt).cos();
            let sin_a = (self.rotation_speed * dt).sin();

            let new_x = self.camera.view_direction.x * cos_a - self.camera.view_direction.y * sin_a;
            let new_y = self.camera.view_direction.x * sin_a + self.camera.view_direction.y * cos_a;

            self.camera.view_direction = Vec2::new(new_x, new_y).normalize();
        }

        let mut move_input = Vec2::ZERO;

        if input.is_key_pressed(KeyCode::Char('w')) {
            move_input += self.camera.view_direction;
        }

        if input.is_key_pressed(KeyCode::Char('s')) {
            move_input -= self.camera.view_direction;
        }

        let right = Vec2::new(self.camera.view_direction.y, -self.camera.view_direction.x);

        if input.is_key_pressed(KeyCode::Char('q')) {
            move_input -= right;
        }

        if input.is_key_pressed(KeyCode::Char('e')) {
            move_input += right;
        }

        if move_input.length() > 0.0 {
            move_input = move_input.normalize();

            self.velocity += move_input * self.acceleration * dt;
        }

        self.velocity *= (1.0 - self.friction * dt).max(0.0);

        if input.is_key_down(KeyCode::Char(' ')) && self.jump_offset == 0.0 {
            self.jump_velocity = self.jump_strength;

            if self.velocity.length() > 0.01 {
                self.velocity += self.camera.view_direction * 1.8;
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

        if !map.is_wall(new_x, self.position.y) {
            self.position.x = new_x;
        } else {
            self.velocity.x = 0.0;
        }

        if !map.is_wall(self.position.x, new_y) {
            self.position.y = new_y;
        } else {
            self.velocity.y = 0.0;
        }
    }
}
