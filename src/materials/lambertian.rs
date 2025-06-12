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
pub struct Lambertian<R: Rng> {
    attenuation: Vector3,
    rng: Rc<RefCell<R>>
}

impl<R: Rng> Lambertian<R> {
    pub fn new(
        attenuation: Vector3,
        rng: Rc<RefCell<R>>
    ) -> Self {
        Self { attenuation, rng }
    }
}

impl<R: Rng> Material for Lambertian<R> {
    fn attenuation(&self, _r: Ray, _t: f64, _n: Vector3) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, r: Ray, t: f64, n: Vector3) -> Ray {
        let rng_ref = &mut self.rng.borrow_mut();
        Ray::new(r.at(t), sample_unit_sphere_uniform(rng_ref) + n)
    }
}