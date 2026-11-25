use crate::raycaster::{Hit, Side};
use crate::render_target::{RenderTarget, Rgb};

const CEILING: Rgb = [210, 205, 150];
const FLOOR: Rgb = [115, 107, 46];
const WALL_X: Rgb = [139, 132, 82];
const WALL_Y: Rgb = [108, 102, 63];

pub fn render(target: &mut dyn RenderTarget, hits: &[Option<Hit>], fov: f32) {
    let width = target.width();
    let height = target.height();
    let screen_height = height as f32;
    let screen_center = screen_height / 2.0;

    for y in 0..height {
        let color = if (y as f32) < screen_center {
            CEILING
        } else {
            FLOOR
        };

        for x in 0..width {
            target.set_pixel(x, y, color);
        }
    }

    let projection_dist = (width as f32 / 2.0) / (fov / 2.0).tan();

    for (x, hit) in hits.iter().enumerate() {
        let Some(hit) = hit else {
            continue;
        };

        let wall_height = (projection_dist / hit.distance.max(0.0001)).min(screen_height * 2.0);

        let wall_start = (screen_center - wall_height / 2.0).max(0.0) as u32;
        let wall_end = (screen_center + wall_height / 2.0).min(screen_height) as u32;

        let color = match hit.side {
            Side::X => WALL_X,
            Side::Y => WALL_Y,
        };

        for y in wall_start..wall_end {
            target.set_pixel(x as u32, y, color);
        }
    }

    target.present();
}
