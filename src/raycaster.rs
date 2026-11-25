use glam::Vec2;

use crate::map::Map;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    X,
    Y,
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub distance: f32,
    pub side: Side,
}

pub fn cast_ray(origin: Vec2, direction: Vec2, map: &Map) -> Option<Hit> {
    let mut cx = origin.x.floor() as i32;
    let mut cy = origin.y.floor() as i32;

    let delta_x = (1.0 / direction.x).abs();
    let delta_y = (1.0 / direction.y).abs();

    let (step_x, mut side_x) = if direction.x < 0.0 {
        (-1, (origin.x - cx as f32) * delta_x)
    } else {
        (1, (cx as f32 + 1.0 - origin.x) * delta_x)
    };

    let (step_y, mut side_y) = if direction.y < 0.0 {
        (-1, (origin.y - cy as f32) * delta_y)
    } else {
        (1, (cy as f32 + 1.0 - origin.y) * delta_y)
    };

    for _ in 0..100 {
        let side = if side_x < side_y {
            side_x += delta_x;
            cx += step_x;
            Side::X
        } else {
            side_y += delta_y;
            cy += step_y;
            Side::Y
        };

        if map.is_wall(cx, cy) {
            let distance = match side {
                Side::X => side_x - delta_x,
                Side::Y => side_y - delta_y,
            };

            return Some(Hit { distance, side });
        }
    }

    None
}

pub fn cast_rays(
    origin: Vec2,
    view_direction: Vec2,
    fov: f32,
    resolution: u32,
    map: &Map,
) -> Vec<Option<Hit>> {
    let plane = Vec2::new(-view_direction.y, view_direction.x) * (fov * 0.5).tan();

    (0..resolution)
        .map(|i| {
            let camera_x = 2.0 * (i as f32 / resolution as f32) - 1.0;
            cast_ray(origin, view_direction + plane * camera_x, map)
        })
        .collect()
}
