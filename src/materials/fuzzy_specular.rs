use crate::{
    materials::Material,
    random::sample_unit_sphere_uniform,
    ray::Ray,
    vector3::Vector3,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuzzySpecular {
    attenuation: Vector3,
    fuzzing_radius: f64,
    max_fuzzing_iterations: usize,  // The maximum number of attempts to find the fuzzed reflection direction.
}

impl FuzzySpecular {
    pub fn new(
        attenuation: Vector3,
        fuzzing_radius: f64,
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
    fn attenuation(&self, _rng: &mut R, _r: Ray, _t: f64, _n: Vector3, _is_inside: bool) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, rng: &mut R, r: Ray, t: f64, n: Vector3, _is_inside: bool) -> Option<Ray> {
        // Rejection sampling for the win!
        let direction_specular_normalized = (r.direction - 2.0 * Vector3::dot(r.direction, n) * n).normalize();
        let mut direction: Vector3;
        for _ in 0..self.max_fuzzing_iterations {
            direction = direction_specular_normalized + self.fuzzing_radius * sample_unit_sphere_uniform(rng);
            if direction.dot(n) > 0.0 {
                return Some(Ray::new(r.at(t), direction));
            }
        }
        None
    }
}