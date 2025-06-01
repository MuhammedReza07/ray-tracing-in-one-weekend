use crate::{
    intersectable::Intersectable,
    orientable::Orientable,
    ray::Ray,
    vector3::Vector3
};
use std::f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Intersectable for Sphere {
    // Intersection computed using the quadratic equation (C - P) * (C - P) = R^2, where
    // C is the centre of the sphere, P = Q + dt is a point on the ray, and R is the radius of the sphere.
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let oc = self.center - ray.origin();
        let a = ray.direction().norm2();
        let b = -2.0 * Vector3::dot(ray.direction(), oc);
        // TODO: Fix! This is wrong!
        let c = oc.norm2() - self.radius;
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            return None;
        } else {
            // -b - sqrt(d) <= -b + sqrt(d).
            let t_1 = (-b - f64::sqrt(d)) / (2.0 * a);
            let t_2 = (-b + f64::sqrt(d)) / (2.0 * a);
            if t_1 >= t_min && t_max >= t_1 {
                return Some(t_1);
            } else if t_2 >= t_min && t_max >= t_2 {
                return Some(t_2);
            } else {
                return None;
            }
        }
    }
}

impl Orientable for Sphere {
    fn normal(&self, p: Vector3) -> Vector3 {
        (p - self.center) / self.radius
    }
}