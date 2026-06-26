#![allow(dead_code)]

// Portions adapted from FastNoiseLite
// https://github.com/Auburn/FastNoiseLite
//
// Copyright (c) 2023 Jordan Peck
// Copyright (c) 2023 Contributors
//
// Licensed under the MIT License.

const PRIME_X: i32 = 501125321;
const PRIME_Y: i32 = 1136930381;
const PRIME_Z: i32 = 1720413743;

#[rustfmt::skip]
const GRADIENTS_2D: [f32; 256] = [
     0.130526192220052,  0.99144486137381,   0.38268343236509,   0.923879532511287,  0.608761429008721,  0.793353340291235,  0.793353340291235,  0.608761429008721,
     0.923879532511287,  0.38268343236509,   0.99144486137381,   0.130526192220051,  0.99144486137381,  -0.130526192220051,  0.923879532511287, -0.38268343236509,
     0.793353340291235, -0.60876142900872,   0.608761429008721, -0.793353340291235,  0.38268343236509,  -0.923879532511287,  0.130526192220052, -0.99144486137381,
    -0.130526192220052, -0.99144486137381,  -0.38268343236509,  -0.923879532511287, -0.608761429008721, -0.793353340291235, -0.793353340291235, -0.608761429008721,
    -0.923879532511287, -0.38268343236509,  -0.99144486137381,  -0.130526192220052, -0.99144486137381,   0.130526192220051, -0.923879532511287,  0.38268343236509,
    -0.793353340291235,  0.608761429008721, -0.608761429008721,  0.793353340291235, -0.38268343236509,   0.923879532511287, -0.130526192220052,  0.99144486137381,
     0.130526192220052,  0.99144486137381,   0.38268343236509,   0.923879532511287,  0.608761429008721,  0.793353340291235,  0.793353340291235,  0.608761429008721,
     0.923879532511287,  0.38268343236509,   0.99144486137381,   0.130526192220051,  0.99144486137381,  -0.130526192220051,  0.923879532511287, -0.38268343236509,
     0.793353340291235, -0.60876142900872,   0.608761429008721, -0.793353340291235,  0.38268343236509,  -0.923879532511287,  0.130526192220052, -0.99144486137381,
    -0.130526192220052, -0.99144486137381,  -0.38268343236509,  -0.923879532511287, -0.608761429008721, -0.793353340291235, -0.793353340291235, -0.608761429008721,
    -0.923879532511287, -0.38268343236509,  -0.99144486137381,  -0.130526192220052, -0.99144486137381,   0.130526192220051, -0.923879532511287,  0.38268343236509,
    -0.793353340291235,  0.608761429008721, -0.608761429008721,  0.793353340291235, -0.38268343236509,   0.923879532511287, -0.130526192220052,  0.99144486137381,
     0.130526192220052,  0.99144486137381,   0.38268343236509,   0.923879532511287,  0.608761429008721,  0.793353340291235,  0.793353340291235,  0.608761429008721,
     0.923879532511287,  0.38268343236509,   0.99144486137381,   0.130526192220051,  0.99144486137381,  -0.130526192220051,  0.923879532511287, -0.38268343236509,
     0.793353340291235, -0.60876142900872,   0.608761429008721, -0.793353340291235,  0.38268343236509,  -0.923879532511287,  0.130526192220052, -0.99144486137381,
    -0.130526192220052, -0.99144486137381,  -0.38268343236509,  -0.923879532511287, -0.608761429008721, -0.793353340291235, -0.793353340291235, -0.608761429008721,
    -0.923879532511287, -0.38268343236509,  -0.99144486137381,  -0.130526192220052, -0.99144486137381,   0.130526192220051, -0.923879532511287,  0.38268343236509,
    -0.793353340291235,  0.608761429008721, -0.608761429008721,  0.793353340291235, -0.38268343236509,   0.923879532511287, -0.130526192220052,  0.99144486137381,
     0.130526192220052,  0.99144486137381,   0.38268343236509,   0.923879532511287,  0.608761429008721,  0.793353340291235,  0.793353340291235,  0.608761429008721,
     0.923879532511287,  0.38268343236509,   0.99144486137381,   0.130526192220051,  0.99144486137381,  -0.130526192220051,  0.923879532511287, -0.38268343236509,
     0.793353340291235, -0.60876142900872,   0.608761429008721, -0.793353340291235,  0.38268343236509,  -0.923879532511287,  0.130526192220052, -0.99144486137381,
    -0.130526192220052, -0.99144486137381,  -0.38268343236509,  -0.923879532511287, -0.608761429008721, -0.793353340291235, -0.793353340291235, -0.608761429008721,
    -0.923879532511287, -0.38268343236509,  -0.99144486137381,  -0.130526192220052, -0.99144486137381,   0.130526192220051, -0.923879532511287,  0.38268343236509,
    -0.793353340291235,  0.608761429008721, -0.608761429008721,  0.793353340291235, -0.38268343236509,   0.923879532511287, -0.130526192220052,  0.99144486137381,
     0.130526192220052,  0.99144486137381,   0.38268343236509,   0.923879532511287,  0.608761429008721,  0.793353340291235,  0.793353340291235,  0.608761429008721,
     0.923879532511287,  0.38268343236509,   0.99144486137381,   0.130526192220051,  0.99144486137381,  -0.130526192220051,  0.923879532511287, -0.38268343236509,
     0.793353340291235, -0.60876142900872,   0.608761429008721, -0.793353340291235,  0.38268343236509,  -0.923879532511287,  0.130526192220052, -0.99144486137381,
    -0.130526192220052, -0.99144486137381,  -0.38268343236509,  -0.923879532511287, -0.608761429008721, -0.793353340291235, -0.793353340291235, -0.608761429008721,
    -0.923879532511287, -0.38268343236509,  -0.99144486137381,  -0.130526192220052, -0.99144486137381,   0.130526192220051, -0.923879532511287,  0.38268343236509,
    -0.793353340291235,  0.608761429008721, -0.608761429008721,  0.793353340291235, -0.38268343236509,   0.923879532511287, -0.130526192220052,  0.99144486137381,
     0.38268343236509,   0.923879532511287,  0.923879532511287,  0.38268343236509,   0.923879532511287, -0.38268343236509,   0.38268343236509,  -0.923879532511287,
    -0.38268343236509,  -0.923879532511287, -0.923879532511287, -0.38268343236509,  -0.923879532511287,  0.38268343236509,  -0.38268343236509,   0.923879532511287,
];

#[rustfmt::skip]
const GRADIENTS_3D: [f32; 256] = [
    0.0, 1.0, 1.0, 0.0,  0.0,-1.0, 1.0, 0.0,  0.0, 1.0,-1.0, 0.0,  0.0,-1.0,-1.0, 0.0,
    1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,  1.0, 0.0,-1.0, 0.0, -1.0, 0.0,-1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0,  1.0,-1.0, 0.0, 0.0, -1.0,-1.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 0.0,  0.0,-1.0, 1.0, 0.0,  0.0, 1.0,-1.0, 0.0,  0.0,-1.0,-1.0, 0.0,
    1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,  1.0, 0.0,-1.0, 0.0, -1.0, 0.0,-1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0,  1.0,-1.0, 0.0, 0.0, -1.0,-1.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 0.0,  0.0,-1.0, 1.0, 0.0,  0.0, 1.0,-1.0, 0.0,  0.0,-1.0,-1.0, 0.0,
    1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,  1.0, 0.0,-1.0, 0.0, -1.0, 0.0,-1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0,  1.0,-1.0, 0.0, 0.0, -1.0,-1.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 0.0,  0.0,-1.0, 1.0, 0.0,  0.0, 1.0,-1.0, 0.0,  0.0,-1.0,-1.0, 0.0,
    1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,  1.0, 0.0,-1.0, 0.0, -1.0, 0.0,-1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0,  1.0,-1.0, 0.0, 0.0, -1.0,-1.0, 0.0, 0.0,
    0.0, 1.0, 1.0, 0.0,  0.0,-1.0, 1.0, 0.0,  0.0, 1.0,-1.0, 0.0,  0.0,-1.0,-1.0, 0.0,
    1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0,  1.0, 0.0,-1.0, 0.0, -1.0, 0.0,-1.0, 0.0,
    1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0,  1.0,-1.0, 0.0, 0.0, -1.0,-1.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 0.0,  0.0,-1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0,  0.0,-1.0,-1.0, 0.0,
];

fn grad_2d(seed: i32, xp: i32, yp: i32, xd: f32, yd: f32) -> f32 {
    let h = (seed ^ xp ^ yp).wrapping_mul(0x27d4eb2d);
    let h = (h ^ (h >> 15)) & (127 << 1);
    xd * GRADIENTS_2D[h as usize] + yd * GRADIENTS_2D[(h | 1) as usize]
}

fn grad_3d(seed: i32, xp: i32, yp: i32, zp: i32, xd: f32, yd: f32, zd: f32) -> f32 {
    let h = (seed ^ xp ^ yp ^ zp).wrapping_mul(0x27d4eb2d);
    let h = (h ^ (h >> 15)) & (63 << 2);
    xd * GRADIENTS_3D[h as usize]
        + yd * GRADIENTS_3D[(h | 1) as usize]
        + zd * GRADIENTS_3D[(h | 2) as usize]
}

pub fn get_simplex_2d(seed: i32, x: f32, y: f32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    const G2: f32 = (3.0 - SQRT3) / 6.0;

    let t = (x + y) * F2;
    let x = x + t;
    let y = y + t;

    let i = x.floor() as i32;
    let j = y.floor() as i32;
    let xi = x - i as f32;
    let yi = y - j as f32;
    let t = (xi + yi) * G2;
    let x0 = xi - t;
    let y0 = yi - t;

    let ip = i.wrapping_mul(PRIME_X);
    let jp = j.wrapping_mul(PRIME_Y);

    let a = 0.5 - x0 * x0 - y0 * y0;
    let n0 = if a <= 0.0 {
        0.0
    } else {
        (a * a) * (a * a) * grad_2d(seed, ip, jp, x0, y0)
    };

    let c = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t
        + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a);
    let n2 = if c <= 0.0 {
        0.0
    } else {
        let x2 = x0 + (2.0 * G2 - 1.0);
        let y2 = y0 + (2.0 * G2 - 1.0);
        (c * c)
            * (c * c)
            * grad_2d(
                seed,
                ip.wrapping_add(PRIME_X),
                jp.wrapping_add(PRIME_Y),
                x2,
                y2,
            )
    };

    let n1 = if y0 > x0 {
        let x1 = x0 + G2;
        let y1 = y0 + (G2 - 1.0);
        let b = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            0.0
        } else {
            (b * b) * (b * b) * grad_2d(seed, ip, jp.wrapping_add(PRIME_Y), x1, y1)
        }
    } else {
        let x1 = x0 + (G2 - 1.0);
        let y1 = y0 + G2;
        let b = 0.5 - x1 * x1 - y1 * y1;
        if b <= 0.0 {
            0.0
        } else {
            (b * b) * (b * b) * grad_2d(seed, ip.wrapping_add(PRIME_X), jp, x1, y1)
        }
    };

    (n0 + n1 + n2) * 99.83685446303647
}

pub fn get_smooth_simplex_2d(seed: i32, x: f32, y: f32) -> f32 {
    const SQRT3: f32 = 1.7320508075688772;
    const F2: f32 = 0.5 * (SQRT3 - 1.0);
    const G2: f32 = (3.0 - SQRT3) / 6.0;

    let t = (x + y) * F2;
    let x = x + t;
    let y = y + t;

    let i = x.floor() as i32;
    let j = y.floor() as i32;
    let xi = x - i as f32;
    let yi = y - j as f32;
    let t = (xi + yi) * G2;
    let x0 = xi - t;
    let y0 = yi - t;

    let ip = i.wrapping_mul(PRIME_X);
    let jp = j.wrapping_mul(PRIME_Y);
    let i1 = ip.wrapping_add(PRIME_X);
    let j1 = jp.wrapping_add(PRIME_Y);

    let a0 = (2.0 / 3.0) - x0 * x0 - y0 * y0;
    let mut v = (a0 * a0) * (a0 * a0) * grad_2d(seed, ip, jp, x0, y0);

    let a1 = (2.0 * (1.0 - 2.0 * G2) * (1.0 / G2 - 2.0)) * t
        + ((-2.0 * (1.0 - 2.0 * G2) * (1.0 - 2.0 * G2)) + a0);
    let x1 = x0 - (1.0 - 2.0 * G2);
    let y1 = y0 - (1.0 - 2.0 * G2);
    v += (a1 * a1) * (a1 * a1) * grad_2d(seed, i1, j1, x1, y1);

    let xmyi = xi - yi;
    if t > G2 {
        if xi + xmyi > 1.0 {
            let x2 = x0 + (3.0 * G2 - 2.0);
            let y2 = y0 + (3.0 * G2 - 1.0);
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2)
                    * (a2 * a2)
                    * grad_2d(
                        seed,
                        ip.wrapping_add(PRIME_X << 1),
                        jp.wrapping_add(PRIME_Y),
                        x2,
                        y2,
                    );
            }
        } else {
            let x2 = x0 + G2;
            let y2 = y0 + (G2 - 1.0);
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2) * (a2 * a2) * grad_2d(seed, ip, jp.wrapping_add(PRIME_Y), x2, y2);
            }
        }
        if yi - xmyi > 1.0 {
            let x3 = x0 + (3.0 * G2 - 1.0);
            let y3 = y0 + (3.0 * G2 - 2.0);
            let a3 = (2.0 / 3.0) - x3 * x3 - y3 * y3;
            if a3 > 0.0 {
                v += (a3 * a3)
                    * (a3 * a3)
                    * grad_2d(
                        seed,
                        ip.wrapping_add(PRIME_X),
                        jp.wrapping_add(PRIME_Y << 1),
                        x3,
                        y3,
                    );
            }
        } else {
            let x3 = x0 + (G2 - 1.0);
            let y3 = y0 + G2;
            let a3 = (2.0 / 3.0) - x3 * x3 - y3 * y3;
            if a3 > 0.0 {
                v += (a3 * a3) * (a3 * a3) * grad_2d(seed, ip.wrapping_add(PRIME_X), jp, x3, y3);
            }
        }
    } else {
        if xi + xmyi < 0.0 {
            let x2 = x0 + (1.0 - G2);
            let y2 = y0 - G2;
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2) * (a2 * a2) * grad_2d(seed, ip.wrapping_sub(PRIME_X), jp, x2, y2);
            }
        } else {
            let x2 = x0 + (G2 - 1.0);
            let y2 = y0 + G2;
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2) * (a2 * a2) * grad_2d(seed, ip.wrapping_add(PRIME_X), jp, x2, y2);
            }
        }
        if yi < xmyi {
            let x2 = x0 - G2;
            let y2 = y0 - (G2 - 1.0);
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2) * (a2 * a2) * grad_2d(seed, ip, jp.wrapping_sub(PRIME_Y), x2, y2);
            }
        } else {
            let x2 = x0 + G2;
            let y2 = y0 + (G2 - 1.0);
            let a2 = (2.0 / 3.0) - x2 * x2 - y2 * y2;
            if a2 > 0.0 {
                v += (a2 * a2) * (a2 * a2) * grad_2d(seed, ip, jp.wrapping_add(PRIME_Y), x2, y2);
            }
        }
    }

    v * 18.24196194486065
}

pub fn fbm_2d(
    seed: i32,
    x: f32,
    z: f32,
    octaves: u32,
    frequency: f32,
    lacunarity: f32,
    gain: f32,
) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut freq = frequency;
    let mut amplitude_sum = 0.0;

    for octave in 0..octaves {
        value += get_smooth_simplex_2d(seed + octave as i32, x * freq, z * freq) * amplitude;
        amplitude_sum += amplitude;
        amplitude *= gain;
        freq *= lacunarity;
    }

    value / amplitude_sum
}

pub fn get_simplex_3d(seed: i32, x: f32, y: f32, z: f32) -> f32 {
    let r = (x + y + z) * (2.0 / 3.0);
    let mut x0 = r - x;
    let mut y0 = r - y;
    let mut z0 = r - z;

    let i = x0.round() as i32;
    let j = y0.round() as i32;
    let k = z0.round() as i32;
    x0 -= i as f32;
    y0 -= j as f32;
    z0 -= k as f32;

    let mut xns = (-1.0 - x0) as i32 | 1;
    let mut yns = (-1.0 - y0) as i32 | 1;
    let mut zns = (-1.0 - z0) as i32 | 1;
    let mut ax0 = xns as f32 * -x0;
    let mut ay0 = yns as f32 * -y0;
    let mut az0 = zns as f32 * -z0;

    let mut ip = i.wrapping_mul(PRIME_X);
    let mut jp = j.wrapping_mul(PRIME_Y);
    let mut kp = k.wrapping_mul(PRIME_Z);

    let mut value = 0.0f32;
    let mut a = (0.6 - x0 * x0) - (y0 * y0 + z0 * z0);
    let mut seed = seed;

    for l in 0..2 {
        if a > 0.0 {
            value += (a * a) * (a * a) * grad_3d(seed, ip, jp, kp, x0, y0, z0);
        }

        if ax0 >= ay0 && ax0 >= az0 {
            let b = a + ax0 + ax0;
            if b > 1.0 {
                let b = b - 1.0;
                value += (b * b)
                    * (b * b)
                    * grad_3d(
                        seed,
                        ip.wrapping_sub(xns.wrapping_mul(PRIME_X)),
                        jp,
                        kp,
                        x0 + xns as f32,
                        y0,
                        z0,
                    );
            }
        } else if ay0 > ax0 && ay0 >= az0 {
            let b = a + ay0 + ay0;
            if b > 1.0 {
                let b = b - 1.0;
                value += (b * b)
                    * (b * b)
                    * grad_3d(
                        seed,
                        ip,
                        jp.wrapping_sub(yns.wrapping_mul(PRIME_Y)),
                        kp,
                        x0,
                        y0 + yns as f32,
                        z0,
                    );
            }
        } else {
            let b = a + az0 + az0;
            if b > 1.0 {
                let b = b - 1.0;
                value += (b * b)
                    * (b * b)
                    * grad_3d(
                        seed,
                        ip,
                        jp,
                        kp.wrapping_sub(zns.wrapping_mul(PRIME_Z)),
                        x0,
                        y0,
                        z0 + zns as f32,
                    );
            }
        }

        if l == 1 {
            break;
        }

        ax0 = 0.5 - ax0;
        ay0 = 0.5 - ay0;
        az0 = 0.5 - az0;
        x0 = xns as f32 * ax0;
        y0 = yns as f32 * ay0;
        z0 = zns as f32 * az0;
        a += (0.75 - ax0) - (ay0 + az0);
        ip = ip.wrapping_add((xns >> 1) & PRIME_X);
        jp = jp.wrapping_add((yns >> 1) & PRIME_Y);
        kp = kp.wrapping_add((zns >> 1) & PRIME_Z);
        xns = -xns;
        yns = -yns;
        zns = -zns;
        seed = !seed;
    }

    value * 32.69428253173828125
}

#[inline]
fn hash(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^= x >> 16;
    x
}

pub fn white_noise_1d(seed: i32, x: i32) -> f32 {
    let h = hash(seed as u32 ^ x.wrapping_mul(PRIME_X) as u32);
    (h as f32) * (2.0 / u32::MAX as f32) - 1.0
}

pub fn white_noise_2d(seed: i32, x: i32, y: i32) -> f32 {
    let h = hash(seed as u32 ^ x.wrapping_mul(PRIME_X) as u32 ^ y.wrapping_mul(PRIME_Y) as u32);
    (h as f32) * (2.0 / u32::MAX as f32) - 1.0
}

pub fn white_noise_3d(seed: i32, x: i32, y: i32, z: i32) -> f32 {
    let h = hash(
        seed as u32
            ^ x.wrapping_mul(PRIME_X) as u32
            ^ y.wrapping_mul(PRIME_Y) as u32
            ^ z.wrapping_mul(PRIME_Z) as u32,
    );
    (h as f32) * (2.0 / u32::MAX as f32) - 1.0
}

pub fn get_voronoi_2d(seed: i32, x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;

    let mut min_dist = f32::MAX;

    for cy in -1..=1 {
        for cx in -1..=1 {
            let cx_ = xi + cx;
            let cy_ = yi + cy;

            let hx = hash(
                seed as u32 ^ cx_.wrapping_mul(PRIME_X) as u32 ^ cy_.wrapping_mul(PRIME_Y) as u32,
            );
            let hy = hash(hx ^ 0xdeadbeef);

            let fx = cx as f32 + hx as f32 * (1.0 / u32::MAX as f32);
            let fy = cy as f32 + hy as f32 * (1.0 / u32::MAX as f32);

            let dx = x - (xi as f32 + fx);
            let dy = y - (yi as f32 + fy);

            let dist = dx * dx + dy * dy;
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }

    min_dist.min(1.0)
}

pub fn get_voronoi_3d(seed: i32, x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    let mut min_dist = f32::MAX;

    for cz in -1..=1 {
        for cy in -1..=1 {
            for cx in -1..=1 {
                let cx_ = xi + cx;
                let cy_ = yi + cy;
                let cz_ = zi + cz;

                let hx = hash(
                    seed as u32
                        ^ cx_.wrapping_mul(PRIME_X) as u32
                        ^ cy_.wrapping_mul(PRIME_Y) as u32
                        ^ cz_.wrapping_mul(PRIME_Z) as u32,
                );
                let hy = hash(hx ^ 0xdeadbeef);
                let hz = hash(hy ^ 0xcafebabe);

                let fx = cx as f32 + hx as f32 * (1.0 / u32::MAX as f32);
                let fy = cy as f32 + hy as f32 * (1.0 / u32::MAX as f32);
                let fz = cz as f32 + hz as f32 * (1.0 / u32::MAX as f32);

                let dx = x - (xi as f32 + fx);
                let dy = y - (yi as f32 + fy);
                let dz = z - (zi as f32 + fz);

                let dist = dx * dx + dy * dy + dz * dz;
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }

    min_dist.min(1.0)
}

pub fn fbm_voronoi_2d(
    seed: i32,
    x: f32,
    z: f32,
    octaves: u32,
    frequency: f32,
    lacunarity: f32,
    gain: f32,
) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut freq = frequency;
    let mut amplitude_sum = 0.0;

    for octave in 0..octaves {
        value += get_voronoi_2d(seed + octave as i32, x * freq, z * freq) * amplitude;
        amplitude_sum += amplitude;
        amplitude *= gain;
        freq *= lacunarity;
    }

    (value / amplitude_sum).min(1.0)
}

pub fn get_voronoi_edges_2d(seed: i32, x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;

    let mut f1 = f32::MAX;
    let mut f2 = f32::MAX;

    for cy in -1..=1 {
        for cx in -1..=1 {
            let cx_ = xi + cx;
            let cy_ = yi + cy;

            let hx = hash(
                seed as u32 ^ cx_.wrapping_mul(PRIME_X) as u32 ^ cy_.wrapping_mul(PRIME_Y) as u32,
            );
            let hy = hash(hx ^ 0xdeadbeef);

            let fx = cx as f32 + hx as f32 * (1.0 / u32::MAX as f32);
            let fy = cy as f32 + hy as f32 * (1.0 / u32::MAX as f32);

            let dx = x - (xi as f32 + fx);
            let dy = y - (yi as f32 + fy);

            let dist = dx * dx + dy * dy;
            if dist < f1 {
                f2 = f1;
                f1 = dist;
            } else if dist < f2 {
                f2 = dist;
            }
        }
    }

    (f2 - f1).min(1.0)
}

pub fn get_voronoi_metric_2d(seed: i32, x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;

    let mut f1 = f32::MAX;
    let mut f2 = f32::MAX;
    let mut closest_x = 0.0f32;
    let mut closest_y = 0.0f32;
    let mut second_x = 0.0f32;
    let mut second_y = 0.0f32;

    for cy in -1..=1 {
        for cx in -1..=1 {
            let cx_ = xi + cx;
            let cy_ = yi + cy;

            let hx = hash(
                seed as u32 ^ cx_.wrapping_mul(PRIME_X) as u32 ^ cy_.wrapping_mul(PRIME_Y) as u32,
            );
            let hy = hash(hx ^ 0xdeadbeef);

            let fx = cx as f32 + hx as f32 * (1.0 / u32::MAX as f32);
            let fy = cy as f32 + hy as f32 * (1.0 / u32::MAX as f32);

            let dx = x - (xi as f32 + fx);
            let dy = y - (yi as f32 + fy);
            let dist = dx * dx + dy * dy;

            if dist < f1 {
                f2 = f1;
                second_x = closest_x;
                second_y = closest_y;
                f1 = dist;
                closest_x = xi as f32 + fx;
                closest_y = yi as f32 + fy;
            } else if dist < f2 {
                f2 = dist;
                second_x = xi as f32 + fx;
                second_y = yi as f32 + fy;
            }
        }
    }

    let ab_x = second_x - closest_x;
    let ab_y = second_y - closest_y;
    let ab_dist = (ab_x * ab_x + ab_y * ab_y).sqrt();

    if ab_dist == 0.0 {
        return 0.0;
    }

    ((f2 - f1) / (2.0 * ab_dist)).clamp(0.0, 1.0)
}
