#![allow(dead_code)]

use glam::{Vec2, Vec3, Vec4};

/// Tests if a point p is inside the triangle defined by points a, b, and c.
/// If the point is inside the triangle, it also calculates the barycentric weights
/// for each vertex of the triangle and returns them through the weight_a, weight_b,
/// and weight_c parameters. The function returns true if the point is inside the triangle
/// and false otherwise.
#[inline(always)]
pub fn point_in_triangle(
    a: Vec2,
    b: Vec2,
    c: Vec2,
    p: Vec2,
    weight_a: &mut f32,
    weight_b: &mut f32,
    weight_c: &mut f32,
) -> bool {
    let e0 = (b - a).perp_dot(p - a);
    let e1 = (c - b).perp_dot(p - b);
    let e2 = (a - c).perp_dot(p - c);

    let has_neg = (e0 < 0.0) | (e1 < 0.0) | (e2 < 0.0);
    let has_pos = (e0 > 0.0) | (e1 > 0.0) | (e2 > 0.0);

    if has_neg & has_pos {
        return false;
    }

    let area = (b - a).perp_dot(c - a);
    if area == 0.0 {
        return false;
    }

    let inv_area = 1.0 / area;

    *weight_a = e1 * inv_area;
    *weight_b = e2 * inv_area;
    *weight_c = e0 * inv_area;

    true
}

/// Converts a Vec4 color to a u32 value representing an RGBA color.
#[inline(always)]
pub fn float4_to_u32_rgba(color: Vec4) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0).round() as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0).round() as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0).round() as u32;
    let a = (color.w.clamp(0.0, 1.0) * 255.0).round() as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Converts a u32 value representing an RGBA color to a Vec4.
#[inline(always)]
pub fn u32_to_float4_rgba(value: u32) -> Vec4 {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    let a = (value & 0xFF) as f32 / 255.0;
    Vec4::new(r, g, b, a)
}

/// Converts a Vec3 color to a u32 value representing an RGB color. The alpha channel is set to 255 (fully opaque).
#[inline(always)]
pub fn float3_to_u32_rgb(color: Vec3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0).round() as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0).round() as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0).round() as u32;
    (r << 24) | (g << 16) | (b << 8) | 0xFF
}

/// Converts a u32 value representing an RGB color to a Vec3.
#[inline(always)]
pub fn u32_to_float3_rgb(value: u32) -> Vec3 {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    Vec3::new(r, g, b)
}

/// Rounds a floating-point number up to the nearest integer and returns it as an i32.
#[inline(always)]
pub fn round_to_int(a: f32) -> i32 {
    a.round() as i32
}

/// Returns the ceiling of a floating-point number as an integer.
#[inline(always)]
pub fn ceil_to_int(a: f32) -> i32 {
    a.ceil() as i32
}

/// Returns the floor of a floating-point number as an integer.
#[inline(always)]
pub fn floor_to_int(a: f32) -> i32 {
    a.floor() as i32
}

/// Performs linear interpolation between two f32 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Performs linear interpolation between two Vec2 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp_float2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Performs linear interpolation between two Vec3 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp_float3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Performs linear interpolation between two Vec4 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp_float4(a: Vec4, b: Vec4, t: f32) -> Vec4 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Maps a value from one range to another range using linear interpolation.
#[inline(always)]
pub fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}
