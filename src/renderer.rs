use glam::{Vec2, Vec3};

use crate::flashlight::Flashlight;
use crate::player::Player;
use crate::raycaster::Hit;
use crate::render_target::{RenderTarget, Rgb};

const AMBIENT: f32 = 0.025;
const WALL_HEIGHT: f32 = 1.0;
const CEILING_ALBEDO: Vec3 = Vec3::new(0.82, 0.8, 0.59);
const FLOOR_ALBEDO: Vec3 = Vec3::new(0.45, 0.42, 0.18);
const WALL_ALBEDO: Vec3 = Vec3::new(0.93, 0.88, 0.55);

fn to_rgb(albedo: Vec3, light: Vec3) -> Rgb {
    let color = (albedo * light).clamp(Vec3::ZERO, Vec3::ONE) * 255.0;
    [color.x as u8, color.y as u8, color.z as u8]
}

pub fn render(
    target: &mut dyn RenderTarget,
    player: &Player,
    hits: &[Option<Hit>],
    flashlight: &Flashlight,
) {
    let width = target.width();
    let height = target.height();
    let screen_height = height as f32;
    let screen_center = screen_height / 2.0;

    let camera = &player.camera;
    let view_direction = camera.view_direction;
    let plane = Vec2::new(-view_direction.y, view_direction.x) * (camera.fov / 2.0).tan();
    let projection_dist = (width as f32 / 2.0) / (camera.fov / 2.0).tan();
    let eye_height = player.eye_height();

    for y in 0..height {
        let offset = y as f32 + 0.5 - screen_center;

        let (albedo, surface_height, normal) = if offset < 0.0 {
            (CEILING_ALBEDO, WALL_HEIGHT, Vec3::NEG_Z)
        } else {
            (FLOOR_ALBEDO, 0.0, Vec3::Z)
        };

        let row_distance =
            (surface_height - eye_height).abs() * projection_dist / offset.abs().max(0.25);

        for x in 0..width {
            let camera_x = 2.0 * (x as f32 / width as f32) - 1.0;
            let ground = player.position + (view_direction + plane * camera_x) * row_distance;
            let point = Vec3::new(ground.x, ground.y, surface_height);

            let light = flashlight.illuminate(AMBIENT, point, normal);
            target.set_pixel(x, y, to_rgb(albedo, light));
        }
    }

    for (x, hit) in hits.iter().enumerate() {
        let Some(hit) = hit else {
            continue;
        };

        let wall_pixels = projection_dist / hit.distance.max(0.0001);

        let wall_top = screen_center - wall_pixels * (WALL_HEIGHT - eye_height);
        let wall_bottom = screen_center + wall_pixels * eye_height;

        let wall_start = wall_top.max(0.0) as u32;
        let wall_end = wall_bottom.min(screen_height) as u32;

        let normal = Vec3::new(hit.normal.x, hit.normal.y, 0.0);

        for y in wall_start..wall_end {
            let z = eye_height - (y as f32 + 0.5 - screen_center) / wall_pixels;
            let point = Vec3::new(hit.position.x, hit.position.y, z);

            let light = flashlight.illuminate(AMBIENT, point, normal);
            target.set_pixel(x as u32, y, to_rgb(WALL_ALBEDO, light));
        }
    }
}
