use crate::vector4::Vector4;
use rand::Rng;
use std::f32::consts::PI;

/// Generates a random point in the unit disk on the plane `z = 0` in `R^3`.
pub fn sample_unit_disk_uniform<R: Rng + ?Sized>(rng: &mut R) -> Vector4 {
    let (r, theta): (f32, f32) = rng.random();
    let r = f32::sqrt(r);
    let theta = 2.0 * PI * theta;
    Vector4::new(r * f32::cos(theta), r * f32::sin(theta), 0.0, 0.0)
}

pub fn sample_unit_sphere_uniform<R: Rng + ?Sized>(rng: &mut R) -> Vector4 {
    let theta = 2.0 * PI * rng.random::<f32>();
    let phi = f32::acos(rng.random_range(-1.0..=1.0));
    Vector4::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos(), 0.0)
}

/// Generates a random point on the hemisphere in the direction of n.
pub fn sample_unit_hemisphere_uniform<R: Rng + ?Sized>(rng: &mut R, n: Vector4) -> Vector4 {
    let sample = sample_unit_sphere_uniform(rng);
    if sample.dot(n) >= 0.0 {
        sample
    } else {
        -sample
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_pcg::Pcg64Mcg;

    #[test]
    fn test_unit_disk_sampling() {
        // Initialise RNG.
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);

        // If the sampling function is uniform, the fraction of samples with
        // r < 1/2 should be 1/4.
        const SAMPLE_COUNT: u32 = 10000;
        const MAX_ERROR: f32 = 0.01;
        let mut r_under_half_count: u32 = 0;
        for _ in 0..SAMPLE_COUNT {
            if sample_unit_disk_uniform(&mut rng).norm() < 0.5 {
                r_under_half_count += 1;
            }
        }
        assert!(f32::abs((r_under_half_count as f32 / SAMPLE_COUNT as f32) - 0.25) < MAX_ERROR);
    }

    #[test]
    fn test_unit_sphere_sampling_norm_1() {
        // Initialise RNG.
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);

        // If the points lie on the unit sphere, their norm should be (approximately) 1.
        const SAMPLE_COUNT: u32 = 1000000;
        const MAX_ERROR: f32 = 0.000000001;
        for _ in 0..SAMPLE_COUNT {
            assert!(f32::abs(1.0 - sample_unit_sphere_uniform(&mut rng).norm()) < MAX_ERROR);
        }
    }

    #[test]
    fn test_unit_sphere_sampling_hemisphere_equidistribution() {
        // Initialise RNG.
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);

        // If the points are uniformly distributed on the sphere, the number of
        // points generated on both hemispheres should be (approximately) equal.
        const SAMPLE_COUNT: u32 = 1000000;
        const MAX_ERROR: f32 = 0.01;
        let n = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let mut samples_in_direction_n_count = 0;
        for _ in 0..SAMPLE_COUNT {
            let s = sample_unit_sphere_uniform(&mut rng);
            if s.dot(n) >= 0.0 {
                samples_in_direction_n_count += 1;
            }
        }
        assert!(f32::abs(1.0 - (SAMPLE_COUNT - samples_in_direction_n_count) as f32 / samples_in_direction_n_count as f32) < MAX_ERROR);
    }

}