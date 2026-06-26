#![allow(dead_code)]

use glam::UVec2;

/// Returns the xy coordinate of a specific index.
#[inline]
pub fn index_to_xy(index: u32, width: u32, height: u32) -> UVec2 {
    if index > width * height {
        return UVec2::new(0, 0);
    }
    let x = index % width;
    let y = index / width;
    UVec2::new(x, y)
}

/// Returns the index of a specific xy coordinate.
#[inline]
pub fn xy_to_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    if x > width || y > height {
        return 0;
    }
    y * width + x
}

/// Converts RGBA u8 channels to a u32 color.
#[inline(always)]
pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

/// Converts a u32 color to RGBA u8 channels.
#[inline(always)]
pub fn u32_to_rgba(value: u32) -> (u8, u8, u8, u8) {
    (
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        value as u8,
    )
}

/// Converts RGB u8 channels to a u32 color. Alpha is set to 255 (fully opaque).
#[inline(always)]
pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF
}

/// Converts a u32 color to RGB u8 channels. Alpha is discarded.
#[inline(always)]
pub fn u32_to_rgb(value: u32) -> (u8, u8, u8) {
    ((value >> 24) as u8, (value >> 16) as u8, (value >> 8) as u8)
}

/// Turns a bool into an u32: true -> 1, false -> 0.
#[inline]
pub fn bool_to_int(a: bool) -> u32 {
    if a { 1 } else { 0 }
}

/// Clamps an u32 between 0 and 1 (returns 0 if a == 0, returns 1 if a != 0).
#[inline]
pub fn clamp01(a: u32) -> u32 {
    a.max(0).min(1)
}

/// Turns a u64 into a binary String.
#[inline]
pub fn int_to_binary_string(value: u64, len: usize) -> String {
    format!("{:0width$b}", value, width = len)
}

/// Turns a u64 into a hexadecimal String.
#[inline]
pub fn int_to_hexadecimal_string(value: u64, len: usize) -> String {
    format!("{:0width$x}", value, width = len)
}

/// Turns a hexadecimal String into a binary String.
#[inline]
pub fn hexadecimal_string_to_binary_string(
    hex: &str,
    len: usize,
) -> Result<String, std::num::ParseIntError> {
    let value = u64::from_str_radix(hex, 16)?;
    Ok(format!("{:0width$b}", value, width = len))
}

/// Turns a binary String into a hexadecimal String.
#[inline]
pub fn binary_string_to_hexadecimal_string(
    bin: &str,
    len: usize,
) -> Result<String, std::num::ParseIntError> {
    let value = u64::from_str_radix(bin, 2)?;
    Ok(format!("{:0width$x}", value, width = len))
}
