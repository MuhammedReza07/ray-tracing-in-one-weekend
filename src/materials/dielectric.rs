use crate::{
    materials::Material,
    ray::Ray,
    vector3::Vector3
};
use rand::Rng;
use std::{
    cell::RefCell, 
    rc::Rc
};

#[derive(Clone, Debug, PartialEq)]
pub struct Dielectric<R: Rng> {
    absorbance: Vector3,
    relative_refractive_index: f64,     // The object's refractive index / the surroundings' refractive index.
    rng: Rc<RefCell<R>>
}

impl<R: Rng> Dielectric<R> {
    pub fn new(
        absorbance: Vector3,
        relative_refractive_index: f64,
        rng: Rc<RefCell<R>>
    ) -> Self {
        Self {
            absorbance,
            relative_refractive_index,
            rng
        }
    }
}

impl<R: Rng> Material for Dielectric<R> {
    fn attenuation(&self, r: Ray, t: f64, _n: Vector3, is_inside: bool) -> Vector3 {
        if is_inside {
            return Vector3::new(
                f64::exp(-self.absorbance.x() * r.length(t)),
                f64::exp(-self.absorbance.y() * r.length(t)),
                f64::exp(-self.absorbance.z() * r.length(t))
            );
        }
        Vector3::new(1.0, 1.0, 1.0)
    }

    fn scatter(&self, r: Ray, t: f64, n: Vector3, is_inside: bool) -> Ray {
        let rng_ref = &mut self.rng.borrow_mut();

        // The relative refractive index must be inverted if the intersection occurred with
        // the ray going into the object.
        let relative_refractive_index = if is_inside {
            self.relative_refractive_index
        } else {
            self.relative_refractive_index.recip()
        };
        let cos_theta_in = r.direction.dot(-n);
        eprintln!("{}", cos_theta_in);
        todo!()
    }
}