use glam::Vec3;

pub struct Spot {
    pub direction: Vec3,
    pub inner_cos: f32,
    pub outer_cos: f32,
}

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub range: f32,
    pub spot: Option<Spot>,
}

impl Light {
    pub fn flashlight(position: Vec3, direction: Vec3) -> Self {
        Self {
            position,
            color: Vec3::new(1.0, 0.95, 0.8) * 5.0,
            range: 15.0,
            spot: Some(Spot {
                direction: direction.normalize(),
                inner_cos: 12f32.to_radians().cos(),
                outer_cos: 28f32.to_radians().cos(),
            }),
        }
    }
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn illuminate(lights: &[Light], ambient: f32, point: Vec3, normal: Vec3) -> Vec3 {
    let mut total = Vec3::splat(ambient);

    for light in lights {
        let to_light = light.position - point;
        let distance = to_light.length();

        if distance >= light.range || distance < 0.0001 {
            continue;
        }

        let direction = to_light / distance;
        let facing = normal.dot(direction);

        if facing <= 0.0 {
            continue;
        }

        let window = (1.0 - (distance / light.range).powi(2)).clamp(0.0, 1.0);
        let attenuation = window * window / (1.0 + distance * distance);

        let cone = match &light.spot {
            Some(spot) => smoothstep(
                spot.outer_cos,
                spot.inner_cos,
                (-direction).dot(spot.direction),
            ),
            None => 1.0,
        };

        total += light.color * (attenuation * cone * facing);
    }

    total
}
