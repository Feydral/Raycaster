use glam::Vec2;

use crate::map::Map;

pub fn cast_ray(origin: Vec2, direction: Vec2, map: &Map) -> Option<f32> {
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
        let hit_x = side_x < side_y;

        if hit_x {
            side_x += delta_x;
            cx += step_x;
        } else {
            side_y += delta_y;
            cy += step_y;
        }

        if map.is_wall(cx, cy) {
            return Some(if hit_x {
                side_x - delta_x
            } else {
                side_y - delta_y
            });
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
) -> Vec<Option<f32>> {
    let plane = Vec2::new(-view_direction.y, view_direction.x) * (fov * 0.5).tan();

    (0..resolution)
        .map(|i| {
            let camera_x = 2.0 * (i as f32 / resolution as f32) - 1.0;
            cast_ray(origin, view_direction + plane * camera_x, map)
        })
        .collect()
}
