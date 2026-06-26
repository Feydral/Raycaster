pub mod hit_info;
pub mod map;

use glam::Vec2;

use crate::raycaster::{hit_info::HitInfo, map::Map};

pub fn cast_rays(
    pos: Vec2,
    view_direction: Vec2,
    fov: f32,
    resolution: u32,
    map: &Map,
) -> Vec<HitInfo> {
    let mut rays = Vec::with_capacity(resolution as usize);

    let dir_len =
        (view_direction.x * view_direction.x + view_direction.y * view_direction.y).sqrt();
    let view_dir = if dir_len > 0.0 {
        Vec2::new(view_direction.x / dir_len, view_direction.y / dir_len)
    } else {
        Vec2::new(1.0, 0.0)
    };

    let half_fov = fov * 0.5;
    let plane_len = half_fov.tan();

    let camera_plane = Vec2::new(-view_dir.y * plane_len, view_dir.x * plane_len);

    for i in 0..resolution {
        let camera_x = 2.0 * (i as f32 / resolution as f32) - 1.0;

        let ray_dir = Vec2::new(
            view_dir.x + camera_plane.x * camera_x,
            view_dir.y + camera_plane.y * camera_x,
        );

        let hit_info = cast_ray(pos, ray_dir, map);
        rays.push(hit_info);
    }

    rays
}

fn cast_ray(pos: Vec2, ray_dir: Vec2, map: &Map) -> HitInfo {
    let dir_x = ray_dir.x;
    let dir_y = ray_dir.y;

    let mut map_x = (pos.x + 1e-6).floor() as i32;
    let mut map_y = (pos.y + 1e-6).floor() as i32;

    let delta_dist_x = if dir_x.abs() < 1e-6 {
        f32::MAX
    } else {
        (1.0 / dir_x).abs()
    };
    let delta_dist_y = if dir_y.abs() < 1e-6 {
        f32::MAX
    } else {
        (1.0 / dir_y).abs()
    };

    let step_x: i32;
    let step_y: i32;

    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (pos.x - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - pos.x) * delta_dist_x;
    }

    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (pos.y - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - pos.y) * delta_dist_y;
    }

    let mut did_hit = false;
    let mut side = 0;

    const MAX_STEPS: u32 = 100;

    for _ in 0..MAX_STEPS {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        if map.is_wall_cell(map_x, map_y) {
            did_hit = true;
            break;
        }
    }

    if !did_hit {
        return HitInfo {
            did_hit: false,
            distance: 0.0,
            axis: 0,
        };
    }

    let distance = if side == 0 {
        side_dist_x - delta_dist_x
    } else {
        side_dist_y - delta_dist_y
    };

    HitInfo {
        did_hit: true,
        distance,
        axis: side,
    }
}
