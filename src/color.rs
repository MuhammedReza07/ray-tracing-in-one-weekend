use std::f64;
use crate::vector3::Vector3;

/// Linearly interpolate from `a` to `b`, `t` must be in `[0, 1]`.
pub fn lerp(a: Vector3, b: Vector3, t: f64) -> Vector3 {
    a + t * (b - a)
}

/// Perform gamma compression on a linear colour component.
pub fn linear_to_gamma(l: f64, encoding_gamma: f64) -> f64 {
    l.powf(encoding_gamma)
}

pub fn write_p3_header(width: u32, height: u32, color_depth: u32) {
    println!("P3\n{} {}\n{}", width, height, color_depth);
}

/// The components of `color` must lie in `[0, 1]`, values outside this range are transformed
/// 
/// to the boundary point they lie closest to (e.g. `-0.34 -> 0.0`, and `2.43 -> 1.0`).
pub fn write_p3_color(color: Vector3, color_depth: u32, encoding_gamma: f64) {
    let r = f64::clamp(color.x(), 0.0, 1.0);
    let g = f64::clamp(color.y(), 0.0, 1.0);
    let b = f64::clamp(color.z(), 0.0, 1.0);
    println!(
        "{} {} {}", 
        (linear_to_gamma(r, encoding_gamma) * color_depth as f64) as u32,
        (linear_to_gamma(g, encoding_gamma) * color_depth as f64) as u32,
        (linear_to_gamma(b, encoding_gamma) * color_depth as f64) as u32
    );
}