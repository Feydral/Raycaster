mod camera;
mod flashlight;
mod map;
mod player;
mod raycaster;
mod render_target;
mod renderer;

use std::time::Instant;

use crossterm::event::KeyCode;
use glam::Vec3;
use termcanvas::prelude::Input;

use crate::flashlight::Flashlight;
use crate::map::Map;
use crate::player::{Player, PlayerInput};
use crate::render_target::RenderTarget;
use crate::render_target::termcanvas::TermcanvasTarget;

fn main() {
    let map = Map::from_rows(&[
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0],
        &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
        &[0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        &[0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
        &[0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        &[0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);

    let mut player = Player::new();

    let mut input = Input::new();
    let mut target = TermcanvasTarget::new();

    let mut last_time = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        input.update().expect("Failed to update input");

        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        player.update(&read_player_input(&input), dt, &map);

        target.sync_size();

        let camera = &player.camera;
        let hits = raycaster::cast_rays(
            player.position,
            camera.view_direction,
            camera.fov,
            target.width(),
            &map,
        );

        let flashlight = Flashlight::new(
            Vec3::new(player.position.x, player.position.y, player.eye_height()),
            Vec3::new(camera.view_direction.x, camera.view_direction.y, 0.0),
        );

        renderer::render(&mut target, &player, &hits, &flashlight);
    }

    target.end();
}

fn axis(input: &Input, positive: char, negative: char) -> f32 {
    let mut value = 0.0;

    if input.is_key_pressed(KeyCode::Char(positive)) {
        value += 1.0;
    }

    if input.is_key_pressed(KeyCode::Char(negative)) {
        value -= 1.0;
    }

    value
}

fn read_player_input(input: &Input) -> PlayerInput {
    PlayerInput {
        forward: axis(input, 'w', 's'),
        turn: axis(input, 'd', 'a'),
        jump: input.is_key_down(KeyCode::Char(' ')),
    }
}
