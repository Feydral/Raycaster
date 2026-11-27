use glam::Vec3;

const BRIGHTNESS: f32 = 9.8;
const RANGE: f32 = 27.0;
const INNER_ANGLE: f32 = 11.0;
const OUTER_ANGLE: f32 = 24.0;

pub struct Flashlight {
    pub position: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub range: f32,
    pub inner_cos: f32,
    pub outer_cos: f32,
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Flashlight {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            color: Vec3::new(1.0, 0.95, 0.8) * BRIGHTNESS,
            range: RANGE,
            inner_cos: INNER_ANGLE.to_radians().cos(),
            outer_cos: OUTER_ANGLE.to_radians().cos(),
        }
    }

    pub fn illuminate(&self, ambient: f32, point: Vec3, normal: Vec3) -> Vec3 {
        let ambient = Vec3::splat(ambient);

        let to_light = self.position - point;
        let distance = to_light.length();

        if distance >= self.range || distance < 0.0001 {
            return ambient;
        }

        let direction = to_light / distance;
        let facing = normal.dot(direction);

        if facing <= 0.0 {
            return ambient;
        }

        let cone = smoothstep(
            self.outer_cos,
            self.inner_cos,
            (-direction).dot(self.direction),
        );

        if cone <= 0.0 {
            return ambient;
        }

        let window = (1.0 - (distance / self.range).powi(2)).clamp(0.0, 1.0);
        let attenuation = window * window / (1.0 + distance * distance);

        ambient + self.color * (attenuation * cone * facing)
    }
}
