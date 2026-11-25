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
    let direction = Vec2::new(1.0, 0.0);

    println!("{:?}", raycaster::cast_ray(origin, direction, &map));
}
