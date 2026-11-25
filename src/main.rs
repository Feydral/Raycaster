mod map;
mod raycaster;

use glam::Vec2;

use crate::map::Map;

fn main() {
    let map = Map {
        width: 5,
        height: 5,
        cells: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    };

    let origin = Vec2::new(0.5, 1.5);
    let view_direction = Vec2::new(1.0, 0.0);

    let distances = raycaster::cast_rays(origin, view_direction, 90f32.to_radians(), 10, &map);

    for (i, distance) in distances.iter().enumerate() {
        println!("{i}: {distance:?}");
    }
}
