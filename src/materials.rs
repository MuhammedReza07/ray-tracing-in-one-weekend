use crate::{
    intersectable::Intersectable,
    orientable::Orientable,
    ray::Ray,
    vector3::Vector3
};
use std::rc::Rc;

/// An "invisible" material that behaves as if there is no material there at all.
pub struct None;

impl Material for None {
    fn attenuation(&self, _r: Ray, _t: f64, _n: Vector3, _is_inside: bool) -> Vector3 {
        // Do not attenuate incoming rays.
        Vector3::new(1.0, 1.0, 1.0)
    }

    fn scatter(&self, r: Ray, t: f64, _n: Vector3, _is_inside: bool) -> Option<Ray> {
        // Do not scatter incoming rays.
        Some(Ray::new(r.at(t), r.direction))
    }
}

/// Trait for tangible objects, i.e. objects consisting of a material.
/// 
/// Note that the `t` value provided to the functions implemented by this trait must yield an intersection between `r` and
/// the implementer for correct behaviour.
pub trait Tangible: Intersectable + Orientable {
    fn material(&self) -> &Rc<dyn Material>;

    fn attenuation(&self, r: Ray, t: f64) -> Vector3 {
        self.material().attenuation(r, t, self.normal(r.at(t)), self.is_inside(r, t))
    }

    fn scatter(&self, r: Ray, t: f64) -> Option<Ray> {
        self.material().scatter(r, t, self.normal(r.at(t)), self.is_inside(r, t))
    }
}

/// Trait defining a common interface for materials.
pub trait Material {
    fn attenuation(&self, r: Ray, t: f64, n: Vector3, is_inside: bool) -> Vector3;

    fn scatter(&self, r: Ray, t: f64, n: Vector3, is_inside: bool) -> Option<Ray>;
}

/// Dielectric material that attenuates rays in accordance with Beer's law.
pub mod dielectric;

/// Non-Lambertian diffuse material that randomly reflects incoming rays.
pub mod diffuse;

/// Specular material with reflected ray fuzzing.
pub mod fuzzy_specular;

/// Lambertian diffuse material.
pub mod lambertian;

/// Specular material, may be used for metals or mirrors.
pub mod specular;