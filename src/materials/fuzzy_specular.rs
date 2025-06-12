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
    rng: Rc<RefCell<R>>
}

impl<R: Rng> FuzzySpecular<R> {
    pub fn new(
        attenuation: Vector3,
        fuzzing_radius: f64,
        rng: Rc<RefCell<R>>
    ) -> Self {
        Self {
            attenuation,
            fuzzing_radius,
            rng
        }
    }
}

impl<R: Rng> Material for FuzzySpecular<R> {
    fn attenuation(&self, _r: Ray, _t: f64, _n: Vector3) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, r: Ray, t: f64, n: Vector3) -> Ray {
        let rng_ref = &mut self.rng.borrow_mut();
        // Rejection sampling for the win!
        let direction_specular_normalized = (r.direction - 2.0 * Vector3::dot(r.direction, n) * n).normalize();
        let mut direction = direction_specular_normalized + self.fuzzing_radius * sample_unit_sphere_uniform(rng_ref);
        while direction.dot(n) <= 0.0 {
            direction = direction_specular_normalized + self.fuzzing_radius * sample_unit_sphere_uniform(rng_ref);
        }
        Ray::new(r.at(t), direction)
    }
}