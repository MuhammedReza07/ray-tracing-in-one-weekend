use crate::{
    materials::Material,
    ray::Ray,
    vector3::Vector3
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Specular {
    attenuation: Vector3
}

impl Specular {
    pub fn new(attenuation: Vector3) -> Self {
        Self { attenuation }
    }
}

impl Material for Specular {
    fn attenuation(&self, _r: Ray, _t: f64, _n: Vector3, _is_inside: bool) -> Vector3 {
        self.attenuation
    }

    fn scatter(&self, r: Ray, t: f64, n: Vector3, _is_inside: bool) -> Option<Ray> {
        Some(Ray::new(r.at(t), r.direction - 2.0 * Vector3::dot(r.direction, n) * n))
    }
}