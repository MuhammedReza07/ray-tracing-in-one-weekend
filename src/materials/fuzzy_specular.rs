use crate::{
    materials::Material,
    random::sample_unit_sphere_uniform,
    ray::Ray,
    vector3::Vector3,
};
use rand::Rng;
use std::{
    cell::RefCell, 
    rc::Rc
};

#[derive(Clone, Debug, PartialEq)]
pub struct FuzzySpecular<R: Rng> {
    attenuation: Vector3,
    fuzzing_radius: f64,
    max_fuzzing_iterations: u32,    // The maximum number of attempts to find the fuzzed reflection direction.
    rng: Rc<RefCell<R>>
}

impl<R: Rng> FuzzySpecular<R> {
    pub fn new(
        attenuation: Vector3,
        fuzzing_radius: f64,
        max_fuzzing_iterations: u32,
        rng: Rc<RefCell<R>>
    ) -> Self {
        Self {
            attenuation,
            fuzzing_radius,
            max_fuzzing_iterations,
            rng
        }
    }
}

impl<R: Rng> Material for FuzzySpecular<R> {
    fn attenuation(&self, _r: Ray, _t: f64, _n: Vector3, _is_inside: bool) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, r: Ray, t: f64, n: Vector3, _is_inside: bool) -> Option<Ray> {
        let rng_ref = &mut self.rng.borrow_mut();
        // Rejection sampling for the win!
        let direction_specular_normalized = (r.direction - 2.0 * Vector3::dot(r.direction, n) * n).normalize();
        let mut direction: Vector3;
        for _ in 0..self.max_fuzzing_iterations {
            direction = direction_specular_normalized + self.fuzzing_radius * sample_unit_sphere_uniform(rng_ref);
            if direction.dot(n) > 0.0 {
                return Some(Ray::new(r.at(t), direction));
            }
        }
        None
    }
}