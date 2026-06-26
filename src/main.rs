mod canvas;
mod input;
mod math;

use crossterm::event::KeyCode;

use crate::{
    canvas::{Canvas, draw::Align, font::Font},
    input::Input,
    math::mathi,
};

fn main() {
    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let font = Font::load_from_file("assets/default.ccfont");
    let bold_font = Font::load_from_file("assets/default_bold.ccfont");

    let mut last_frame = std::time::Instant::now();
    let mut display_fps = 0.0;
    let mut fps = 0;

    loop {
        input.update().expect("failed to update input");

        if input.is_key_pressed(KeyCode::Esc) {
            break;
        }

        let now = std::time::Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        display_fps += dt;

        let new_w = Canvas::terminal_width();
        let new_h = Canvas::terminal_height();

        if new_w as u32 != canvas.width() || new_h as u32 != canvas.height() {
            canvas.resize(new_w, new_h);
        }

        if display_fps > 1.0 {
            display_fps = 0.0;
            fps = (1.0 / dt) as u32;
        }

        canvas.clear();

        let blue = mathi::rgb_to_u32(127, 127, 255);
        let red = mathi::rgb_to_u32(255, 127, 127);

        canvas
            .draw(&bold_font)
            .at(5, 5)
            .color(blue)
            .align(Align::Left)
            .text(&('A'..'Z').collect::<String>());

        canvas
            .draw(&bold_font)
            .at(5, 15)
            .color(blue)
            .align(Align::Left)
            .text(&('a'..'z').collect::<String>());

        canvas
            .draw(&bold_font)
            .at(5, 25)
            .color(blue)
            .align(Align::Left)
            .text(&('0'..='9').collect::<String>());

        canvas
            .draw(&bold_font)
            .at(5, 35)
            .color(blue)
            .align(Align::Left)
            .text(".,:;_-+*#\'\"!?()<>{}[]%&/\\=|");

        canvas
            .draw(&font)
            .at(5, 50)
            .color(red)
            .align(Align::Left)
            .text(&('A'..'Z').collect::<String>());

        canvas
            .draw(&font)
            .at(5, 60)
            .color(red)
            .align(Align::Left)
            .text(&('a'..'z').collect::<String>());

        canvas
            .draw(&font)
            .at(5, 70)
            .color(red)
            .align(Align::Left)
            .text(&('0'..='9').collect::<String>());

        canvas
            .draw(&font)
            .at(5, 80)
            .color(red)
            .align(Align::Left)
            .text(".,:;_-+*#\'\"!?()<>{}[]%&/\\=|");

        let width = canvas.width();
        let height = canvas.height();

        canvas
            .draw(&font)
            .x(width.saturating_sub(5))
            .y(height.saturating_sub(38))
            .color(mathi::rgb_to_u32(127, 255, 127))
            .align(Align::Right)
            .text(format!("Width: {}", width));

        canvas
            .draw(&font)
            .x(width.saturating_sub(5))
            .y(height.saturating_sub(28))
            .color(mathi::rgb_to_u32(127, 255, 127))
            .align(Align::Right)
            .text(format!("Height: {}", height));

        canvas
            .draw(&font)
            .x(width.saturating_sub(5))
            .y(height.saturating_sub(13))
            .color(mathi::rgb_to_u32(255, 255, 127))
            .align(Align::Right)
            .text(format!("FPS: {}", fps));

        canvas.render();
    }

    canvas.end();
}
