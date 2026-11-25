mod camera;
mod map;
mod player;
mod raycaster;
mod render_target;
mod renderer;

use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::event::KeyCode;
use termcanvas::prelude::Input;

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
        renderer::render(&mut target, &hits, camera.fov);

        sleep(Duration::from_millis(16));
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
        strafe: axis(input, 'e', 'q'),
        turn: axis(input, 'd', 'a'),
    }
}
