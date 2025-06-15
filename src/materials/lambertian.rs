use crate::{
    materials::Material,
    random::sample_unit_sphere_uniform,
    ray::Ray,
    vector3::Vector3,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambertian {
    attenuation: Vector3,
}

impl Lambertian {
    pub fn new(
        attenuation: Vector3,
    ) -> Self {
        Self { attenuation }
    }
}

impl<R: Rng + ?Sized> Material<R> for Lambertian {
    fn attenuation(&self, _rng: &mut R, _r: Ray, _t: f64, _n: Vector3, _is_inside: bool) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, rng: &mut R, r: Ray, t: f64, n: Vector3, _is_inside: bool) -> Option<Ray> {
        Some(Ray::new(r.at(t), sample_unit_sphere_uniform(rng) + n))
    }
}