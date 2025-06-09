use std::f64;
use crate::vector3::Vector3;

/// Linearly interpolate from `a` to `b`, `t` must be in `[0, 1]`.
pub fn lerp(a: &Vector3, b: &Vector3, t: f64) -> Vector3 {
    *a + t * (*b - *a)
}

pub fn write_p3_header(width: u32, height: u32, color_depth: u32) {
    println!("P3\n{} {}\n{}", width, height, color_depth);
}

/// The components of `color` must lie in `[0, 1]`, values outside this range are transformed
/// 
/// to the boundary point they lie closest to (e.g. `-0.34 -> 0.0`, and `2.43 -> 1.0`).
pub fn write_p3_color(color: &Vector3, color_depth: u32) {
    let r = (f64::clamp(color.x(), 0.0, 1.0) * color_depth as f64) as u32;
    let g = (f64::clamp(color.y(), 0.0, 1.0) * color_depth as f64) as u32;
    let b = (f64::clamp(color.z(), 0.0, 1.0) * color_depth as f64) as u32;
    println!("{} {} {}", r, g, b);
}