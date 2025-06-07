use crate::vector3::Vector3;
use rand::Rng;
use std::f64;

/// Generates a random point in the unit disk on the plane `z = 0` in `R^3`.
pub fn sample_unit_disk_uniform<R: Rng + ?Sized>(rng: &mut R) -> Vector3 {
    let (r, theta): (f64, f64) = rng.random();
    let r = f64::sqrt(r);
    let theta = 2.0 * f64::consts::PI * theta;
    Vector3::new(r * f64::cos(theta), r * f64::sin(theta), 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_pcg::Pcg64Mcg;
    use std::f64;

    #[test]
    fn test_unit_disk_sampling() {
        // Initialise RNG.
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);

        // If the sampling function is uniform, the fraction of samples with
        // r < 1/2 should be 1/4.
        const SAMPLE_COUNT: u32 = 10000;
        const MAX_ERROR: f64 = 0.01;
        let mut r_under_half_count: u32 = 0;
        for _ in 0..SAMPLE_COUNT {
            if sample_unit_disk_uniform(&mut rng).norm() < 0.5 {
                r_under_half_count += 1
            }
        }
        assert!(f64::abs((r_under_half_count as f64 / SAMPLE_COUNT as f64) - 0.25) < MAX_ERROR);
    }
}