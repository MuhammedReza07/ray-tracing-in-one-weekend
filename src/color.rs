use std::f64;
use crate::vector3::Vector3;

/// Representation of an RGB image.
/// `pixels` should be read in row-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    color_depth: usize,
    encoding_gamma: f64,
    pixels: Vec<Vector3>
}

impl Image {
    pub fn new(
        width: usize,
        height: usize,
        color_depth: usize,
        encoding_gamma: f64,
    ) -> Self {
        let black = Vector3::new(0.0, 0.0, 0.0);
        let mut pixels = Vec::new();
        pixels.reserve_exact(width * height);
        for _ in 0..width * height {
            pixels.push(black);
        }
        Self {
            width,
            height,
            color_depth,
            encoding_gamma,
            pixels
        }
    }

    pub fn set_pixel(&mut self, value: Vector3, i: usize, j: usize) {
        self.pixels[i * self.width + j] = value;
    }

    pub fn set_row(&mut self, values: Vec<Vector3>, i: usize) {
        for j in 0..self.width {
            self.pixels[i * self.width + j] = values[j];
        }
    }

    pub fn write_p3_image_stdout(&self) {
        println!("P3\n{} {}\n{}", self.width, self.height, self.color_depth);
        self.pixels.iter()
        .for_each(|color| {
            let r = f64::clamp(color.x(), 0.0, 1.0);
            let g = f64::clamp(color.y(), 0.0, 1.0);
            let b = f64::clamp(color.z(), 0.0, 1.0);
            println!(
                "{} {} {}", 
                (linear_to_gamma(r, self.encoding_gamma) * self.color_depth as f64) as usize,
                (linear_to_gamma(g, self.encoding_gamma) * self.color_depth as f64) as usize,
                (linear_to_gamma(b, self.encoding_gamma) * self.color_depth as f64) as usize
            );
        });
    }
}

/// Linearly interpolate from `a` to `b`, `t` must be in `[0, 1]`.
pub fn lerp(a: Vector3, b: Vector3, t: f64) -> Vector3 {
    a + t * (b - a)
}

/// Perform gamma compression on a linear colour component.
pub fn linear_to_gamma(l: f64, encoding_gamma: f64) -> f64 {
    l.powf(encoding_gamma)
}