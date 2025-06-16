use crate::ray::Ray;

/// Trait for objects that may be intersected by a ray.
pub trait Intersectable {
    /// Finds the smallest value of `t` such that `r` intersects the object and `t` lies in `[t_min, t_max]`.
    /// 
    /// Returns `Some(t)` if such a `t` is found, `None` otherwise.
    fn intersect(&self, r: Ray, t_min: f32, t_max: f32) -> Option<f32>;
}