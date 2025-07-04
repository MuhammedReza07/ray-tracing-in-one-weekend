use crate::{
    intersectable::Intersectable,
    orientable::Orientable,
    materials::{Material, Tangible},
    ray::Ray,
    vector4::Vector4
};
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere<R: Rng + ?Sized> {
    pub center: Vector4,
    pub radius: f32,
    material: Arc<dyn Material<R> + Send + Sync>
}

impl<R: Rng + ?Sized> Sphere<R> {
    pub fn new(
        center: Vector4, 
        radius: f32,
        material: Arc<dyn Material<R> + Send + Sync>
    ) -> Self {
        Self { center, radius, material }
    }
}

impl<R: Rng + ?Sized> Intersectable for Sphere<R> {
    // Intersection computed using the quadratic equation (C - P) * (C - P) = R^2, where
    // C is the centre of the sphere, P = Q + dt is a point on the ray, and R is the radius of the sphere.
    fn intersect(&self, r: Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let oc = self.center - r.origin;
        let a = r.direction.norm2();
        let b = -2.0 * r.direction.dot(oc);
        let c = oc.norm2() - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            None
        } else {
            // -b - sqrt(d) <= -b + sqrt(d).
            let t_1 = (-b - f32::sqrt(d)) / (2.0 * a);
            let t_2 = (-b + f32::sqrt(d)) / (2.0 * a);
            if t_1 >= t_min && t_max >= t_1 {
                Some(t_1)
            } else if t_2 >= t_min && t_max >= t_2 {
                Some(t_2)
            } else {
                None
            }
        }
    }
}

impl<R: Rng + ?Sized> Orientable for Sphere<R> {
    fn normal(&self, p: Vector4) -> Vector4 {
        (p - self.center) / self.radius
    }
}

impl<R: Rng + ?Sized> Tangible<R> for Sphere<R> {
    fn material(&self) -> &Arc<dyn Material<R> + Send + Sync> {
        &self.material
    }
}