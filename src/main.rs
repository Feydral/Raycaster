mod canvas;
mod game;
mod input;
mod math;
mod raycaster;

use std::time::Instant;

use crossterm::event::KeyCode;

use crate::{game::Game, input::Input};

fn main() {
    let mut game = Game::new();
    let mut input = Input::new();

    let mut last_time = Instant::now();

    game.on_start();

    loop {
        let now = std::time::Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        input.update().expect("Failed to update input");

        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        game.on_update(&input, dt);
        game.on_render();
    }

    game.on_end();
}
