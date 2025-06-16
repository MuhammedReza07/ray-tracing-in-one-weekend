use crate::{
    materials::Material,
    random::sample_unit_sphere_uniform,
    ray::Ray,
    vector4::Vector4,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuzzySpecular {
    attenuation: Vector4,
    fuzzing_radius: f32,
    max_fuzzing_iterations: usize,  // The maximum number of attempts to find the fuzzed reflection direction.
}

impl FuzzySpecular {
    pub fn new(
        attenuation: Vector4,
        fuzzing_radius: f32,
        max_fuzzing_iterations: usize,
    ) -> Self {
        Self {
            attenuation,
            fuzzing_radius,
            max_fuzzing_iterations,
        }
    }
}

impl<R: Rng + ?Sized> Material<R> for FuzzySpecular {
    fn attenuation(&self, _rng: &mut R, _r: Ray, _t: f32, _n: Vector4, _is_inside: bool) -> Vector4 {
        self.attenuation
    }

    fn scatter(&self, rng: &mut R, r: Ray, t: f32, n: Vector4, _is_inside: bool) -> Option<Ray> {
        // Rejection sampling for the win!
        let direction_specular_normalized = (r.direction - 2.0 * r.direction.dot(n) * n).normalize();
        let mut direction: Vector4;
        for _ in 0..self.max_fuzzing_iterations {
            direction = direction_specular_normalized + self.fuzzing_radius * sample_unit_sphere_uniform(rng);
            if direction.dot(n) > 0.0 {
                return Some(Ray::new(r.at(t), direction));
            }
        }
        None
    }
}