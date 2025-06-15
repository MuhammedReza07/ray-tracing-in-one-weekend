use crate::{
    materials::Material,
    ray::Ray,
    vector3::Vector3
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    absorbance: Vector3,
    relative_refractive_index: f64,     // The object's refractive index / the surroundings' refractive index.
}

impl Dielectric {
    pub fn new(
        absorbance: Vector3,
        relative_refractive_index: f64,
    ) -> Self {
        Self {
            absorbance,
            relative_refractive_index,
        }
    }
}

impl<R: Rng + ?Sized> Material<R> for Dielectric {
    fn attenuation(&self, _rng: &mut R, r: Ray, t: f64, _n: Vector3, is_inside: bool) -> Vector3 {
        if is_inside {
            return Vector3::new(
                f64::exp(-self.absorbance.x() * r.length(t)),
                f64::exp(-self.absorbance.y() * r.length(t)),
                f64::exp(-self.absorbance.z() * r.length(t))
            );
        }
        Vector3::new(1.0, 1.0, 1.0)
    }

    fn scatter(&self, rng: &mut R, r: Ray, t: f64, n: Vector3, is_inside: bool) -> Option<Ray> {
        // The relative refractive index must be inverted if the intersection occurred with
        // the ray going into the object.
        let (relative_refractive_index, normal_adjustment) = match is_inside {
            true => (self.relative_refractive_index, 1.0),
            _ => (self.relative_refractive_index.recip(), -1.0)
        };
        let direction_in = r.direction.normalize();
        let local_normal = -normal_adjustment * n;
        let cos_theta_in = normal_adjustment * direction_in.dot(n);

        // Schlick's approximation of dielectric reflectance.
        let r_0 = (1.0 - relative_refractive_index) / (1.0 + relative_refractive_index);
        let r_0 = r_0 * r_0;
        let reflectance = r_0 + (1.0 - r_0) * (1.0 - cos_theta_in) * (1.0 - cos_theta_in) * (1.0 - cos_theta_in) * (1.0 - cos_theta_in) * (1.0 - cos_theta_in);

        // Scatter.
        let sin_theta_in = f64::sqrt(1.0 - cos_theta_in * cos_theta_in);
        if relative_refractive_index * sin_theta_in > 1.0 || rng.random_bool(reflectance) {
            return Some(Ray::new(r.at(t), r.direction - 2.0 * r.direction.dot(local_normal) * local_normal));
        } else {
            let r_out_direction_perp = relative_refractive_index * (direction_in + cos_theta_in * local_normal);
            let r_out_direction_parallel = -local_normal * f64::sqrt(1.0 - r_out_direction_perp.norm2());
            return Some(Ray::new(r.at(t), r_out_direction_perp + r_out_direction_parallel));
        }
    }
}