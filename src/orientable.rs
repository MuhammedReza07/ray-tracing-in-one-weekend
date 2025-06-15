use crate::{ray::Ray, vector3::Vector3};

/// Trait for objects that admit an assignment of surface normals to each point of their surface.
pub trait Orientable {
    /// Returns an outward-facing surface normal of the object at `p`. The returned normal is of unit length.
    /// 
    /// Note that `p` must lie on the surface of the object for this method to behave as intended.
    fn normal(&self, p: Vector3) -> Vector3;

    /// Returns `true` if the intersection of `r` and the object at `t` occurred
    /// with `r` going out from the object, `false` otherwise.
    fn is_inside(&self, r: Ray, t: f64) -> bool {
        let p = r.at(t);
        if r.direction.dot(self.normal(p)) >= 0.0 {
            return true;
        }
        false
    }
}