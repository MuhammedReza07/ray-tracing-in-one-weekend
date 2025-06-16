use crate::{
    materials::Material,
    random::sample_unit_hemisphere_uniform,
    ray::Ray,
    vector4::Vector4,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Diffuse {
    attenuation: Vector4,
}

impl Diffuse {
    pub fn new(
        attenuation: Vector4,
    ) -> Self {
        Self { attenuation }
    }
}

impl<R: Rng + ?Sized> Material<R> for Diffuse {
    fn attenuation(&self, _rng: &mut R, _r: Ray, _t: f32, _n: Vector4, _is_inside: bool) -> Vector4 {
        self.attenuation
    }

    fn scatter(&self, rng: &mut R, r: Ray, t: f32, n: Vector4, _is_inside: bool) -> Option<Ray> {
        Some(Ray::new(r.at(t), sample_unit_hemisphere_uniform(rng, n)))
    }
}