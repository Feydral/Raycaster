pub mod camera;
pub mod player;

use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::SetTitle;

use crate::canvas::Canvas;
use crate::canvas::draw::Align;
use crate::canvas::font::Font;
use crate::game::player::Player;
use crate::input::Input;
use crate::math::mathi;
use crate::raycaster;
use crate::raycaster::map::Map;

pub struct Game {
    canvas: Canvas,
    map: Map,
    player: Player,

    flashlight_radius: f32,
    flashlight_falloff: f32,
    flashlight_brightness: f32,
    flashlight_depth_falloff: f32,
    ambient: f32,

    enable_debug_features: bool,
    depth_buffer: Vec<f32>,

    font: Font,
    font_bold: Font,

    time: f64,
    fps: f32,
    fps_display_timer: f32,
}

impl Game {
    const MAX_DEPTH: f32 = 30.0;

    pub fn on_start() -> Self {
        let mut map = Map::new();
        map.grid = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        Self {
            canvas: Canvas::new(),
            map,
            player: Player::on_start(),

            flashlight_radius: 0.6,
            flashlight_falloff: 1.5,
            flashlight_brightness: 0.4,
            flashlight_depth_falloff: 0.3,
            ambient: 0.03,

            enable_debug_features: false,

            depth_buffer: Vec::new(),

            font: Font::load_from_file("assets/default.ccfont"),
            font_bold: Font::load_from_file("assets/default_bold.ccfont"),

            time: 0.0,
            fps: 0.0,
            fps_display_timer: 1.0,
        }
    }

    pub fn on_update(&mut self, input: &Input, dt: f32) {
        let new_w = Canvas::terminal_width();
        let new_h = Canvas::terminal_height();
        if new_w != self.canvas.width() || new_h != self.canvas.height() {
            self.canvas.resize(new_w, new_h);
        }

        self.player.on_update(input, dt, &self.map);

        if input.is_key_down(KeyCode::Char('1')) {
            self.enable_debug_features = true;
        }
        if input.is_key_down(KeyCode::Char('2')) {
            self.enable_debug_features = false;
        }

        if self.enable_debug_features {
            if input.is_key_pressed(KeyCode::Char('t')) {
                self.flashlight_radius += 0.1 * dt;
            }
            if input.is_key_pressed(KeyCode::Char('g')) {
                self.flashlight_radius -= 0.1 * dt;
            }

            if input.is_key_pressed(KeyCode::Char('z')) {
                self.flashlight_falloff += 1.0 * dt;
            }
            if input.is_key_pressed(KeyCode::Char('h')) {
                self.flashlight_falloff -= 1.0 * dt;
            }

            if input.is_key_pressed(KeyCode::Char('u')) {
                self.flashlight_brightness += 0.5 * dt;
            }
            if input.is_key_pressed(KeyCode::Char('j')) {
                self.flashlight_brightness -= 0.5 * dt;
            }

            if input.is_key_pressed(KeyCode::Char('i')) {
                self.flashlight_depth_falloff += 0.5 * dt;
            }
            if input.is_key_pressed(KeyCode::Char('k')) {
                self.flashlight_depth_falloff -= 0.5 * dt;
            }
        }

        self.time += dt as f64;

        self.fps_display_timer -= dt;
        if self.fps_display_timer <= 0.0 {
            self.fps = 1.0 / dt;
            self.fps_display_timer = 1.0;
        }
    }

    pub fn on_render(&mut self) {
        let hit_infos = raycaster::cast_rays(
            self.player.position,
            self.player.camera.view_direction,
            self.player.camera.fov.to_radians(),
            self.canvas.width(),
            &self.map,
        );

        let projection_dist = (self.canvas.width() as f32 / 2.0)
            / (self.player.camera.fov / 2.0 * std::f32::consts::PI / 180.0).tan();
        let screen_height = self.canvas.height() as f32;
        let screen_center = screen_height / 2.0;

        let horizon =
            screen_center - (projection_dist / Self::MAX_DEPTH * 2.0) * self.player.jump_offset;

        let cx = self.canvas.width() as f32 / 2.0;
        let cy = self.canvas.height() as f32 / 2.0;
        let max_radius = self.canvas.height() as f32 * self.flashlight_radius;

        let total_pixels = (self.canvas.width() * self.canvas.height()) as usize;
        self.depth_buffer.resize(total_pixels, 1.0);

        self.canvas.clear();

        for y in 0..self.canvas.height() {
            let dist_from_horizon = (y as f32 - horizon).abs().max(0.5);
            let floor_distance = projection_dist / dist_from_horizon;
            let depth = (floor_distance / Self::MAX_DEPTH * 0.45).clamp(0.0, 1.0);

            for x in 0..self.canvas.width() {
                if y as f32 > horizon {
                    self.canvas
                        .set_pixel(x, y, mathi::rgb_to_u32(210, 205, 150));
                } else {
                    self.canvas.set_pixel(x, y, mathi::rgb_to_u32(115, 107, 46));
                }
                self.depth_buffer[(y * self.canvas.width() + x) as usize] = depth;
            }
        }

        for x in 0..self.canvas.width() {
            let hit = &hit_infos[x as usize];
            if !hit.did_hit {
                continue;
            }

            let distance = hit.distance.max(0.0001);
            let wall_height = (projection_dist / distance).min(screen_height * 2.0);

            let wall_center = screen_center + wall_height * -self.player.jump_offset;

            let wall_start = (wall_center - wall_height / 2.0).max(0.0) as u32;
            let wall_end = (wall_center + wall_height / 2.0).min(screen_height) as u32;

            let base = 150.0f32;
            let mut r = base;
            let mut g = base;
            let mut b = base;

            match hit.axis {
                0 => {
                    r *= 0.93;
                    g *= 0.88;
                    b *= 0.55;
                }
                1 => {
                    r *= 0.72;
                    g *= 0.68;
                    b *= 0.42;
                }
                _ => {}
            }

            let r = r.min(255.0) as u8;
            let g = g.min(255.0) as u8;
            let b = b.min(255.0) as u8;

            let wall_depth = (distance / Self::MAX_DEPTH).clamp(0.0, 1.0);

            for y in wall_start..wall_end {
                self.canvas.set_pixel(x, y, mathi::rgb_to_u32(r, g, b));
                self.depth_buffer[(y * self.canvas.width() + x) as usize] = wall_depth;
            }
        }

        for y in 0..self.canvas.height() {
            for x in 0..self.canvas.width() {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let screen_dist = (dx * dx + dy * dy).sqrt();

                let depth = self.depth_buffer[(y * self.canvas.width() + x) as usize];
                let depth_factor = (1.0 - depth.powf(self.flashlight_depth_falloff)).max(0.0);

                let shade = smooth_flashlight(
                    screen_dist,
                    max_radius,
                    self.flashlight_falloff,
                    self.flashlight_brightness * depth_factor,
                    self.ambient,
                );

                let pixel = self.canvas.get_pixel(x, y);
                let pr = ((pixel >> 24) & 0xFF) as f32;
                let pg = ((pixel >> 16) & 0xFF) as f32;
                let pb = ((pixel >> 8) & 0xFF) as f32;

                let res_color =
                    mathi::rgb_to_u32((pr * shade) as u8, (pg * shade) as u8, (pb * shade) as u8);
                self.canvas.set_pixel(x, y, res_color);
            }
        }

        if self.enable_debug_features {
            let blue = mathi::rgb_to_u32(0, 127, 255);
            let yellow = mathi::rgb_to_u32(200, 200, 50);
            let bright_yellow = mathi::rgb_to_u32(240, 240, 150);
            let green = mathi::rgb_to_u32(0, 200, 0);

            self.canvas
                .draw(&self.font)
                .at(5, 5)
                .align(Align::Left)
                .color(blue)
                .uint(self.fps as u32);

            let labels = [("R:", 20), ("F:", 30), ("B:", 40), ("D:", 50)];
            for (label, y) in labels {
                self.canvas
                    .draw(&self.font)
                    .at(5, y)
                    .align(Align::Left)
                    .color(bright_yellow)
                    .text(label);
            }

            let values: [f32; 4] = [
                self.flashlight_radius,
                self.flashlight_falloff,
                self.flashlight_brightness,
                self.flashlight_depth_falloff,
            ];
            for (i, val) in values.iter().enumerate() {
                self.canvas
                    .draw(&self.font)
                    .at(15, 20 + i as u32 * 10)
                    .align(Align::Left)
                    .color(yellow)
                    .float(*val, 2, false);
            }

            let x = self.canvas.width() - 5;

            self.canvas
                .draw(&self.font)
                .at(x, 5)
                .align(Align::Right)
                .color(green)
                .float(self.player.position.x, 2, false);

            self.canvas
                .draw(&self.font)
                .at(x, 15)
                .align(Align::Right)
                .color(green)
                .float(self.player.position.y, 2, false);

            let red = mathi::rgb_to_u32(200, 0, 0);

            self.canvas
                .draw(&self.font)
                .at(x, 25)
                .align(Align::Right)
                .color(red)
                .float(
                    self.player
                        .camera
                        .view_direction
                        .y
                        .atan2(self.player.camera.view_direction.x)
                        .to_degrees(),
                    2,
                    false,
                );
        }

        self.canvas.render();
    }

    pub fn on_end(self) {
        self.canvas.end();
    }
}

fn smooth_flashlight(
    dist: f32,
    max_radius: f32,
    falloff: f32,
    brightness: f32,
    ambient: f32,
) -> f32 {
    let t = (dist / max_radius).clamp(0.0, 1.0);
    let shade = 1.0 - t.powf(falloff);
    (shade * brightness).max(ambient)
}
