use crate::vector4::Vector4;

/// Representation of an RGB image.
/// `pixels` should be read in row-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    color_depth: usize,
    encoding_gamma: f32,
    pixels: Vec<Vector4>
}

impl Image {
    pub fn new(
        width: usize,
        height: usize,
        color_depth: usize,
        encoding_gamma: f32,
    ) -> Self {
        let zero_vec = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let mut pixels = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            pixels.push(zero_vec);
        }
        Self {
            width,
            height,
            color_depth,
            encoding_gamma,
            pixels
        }
    }

    pub fn set_pixel(&mut self, value: Vector4, i: usize, j: usize) {
        self.pixels[i * self.width + j] = value;
    }

    pub fn set_row(&mut self, values: &[Vector4], i: usize) {
        for j in 0..self.width {
            self.pixels[i * self.width + j] = values[j];
        }
    }

    pub fn write_p3_image_stdout(&self) {
        println!("P3\n{} {}\n{}", self.width, self.height, self.color_depth);
        self.pixels.iter()
        .for_each(|color| {
            let r = f32::clamp(color.x(), 0.0, 1.0);
            let g = f32::clamp(color.y(), 0.0, 1.0);
            let b = f32::clamp(color.z(), 0.0, 1.0);
            println!(
                "{} {} {}", 
                (linear_to_gamma(r, self.encoding_gamma) * self.color_depth as f32) as usize,
                (linear_to_gamma(g, self.encoding_gamma) * self.color_depth as f32) as usize,
                (linear_to_gamma(b, self.encoding_gamma) * self.color_depth as f32) as usize
            );
        });
    }
}

/// Linearly interpolate from `a` to `b`, `t` must be in `[0, 1]`.
pub fn lerp(a: Vector4, b: Vector4, t: f32) -> Vector4 {
    a + t * (b - a)
}

/// Perform gamma compression on a linear colour component.
pub fn linear_to_gamma(l: f32, encoding_gamma: f32) -> f32 {
    l.powf(encoding_gamma)
}