use std::{
    arch::x86_64::*,
    convert,
    f32,
    fmt,
    ops
};

#[derive(Clone, Copy)]
pub union Vector4 {
    value: [f32; 4],
    simd: __m128
}

impl fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.value) }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp_vec = _mm_cmpeq_ps(self.simd, other.simd);
            _mm_movemask_ps(cmp_vec) == 0xf
        }
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { value: [x, y, z, w] }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.value[0] }
    }

    pub fn y(&self) -> f32 {
        unsafe { self.value[1] }
    }

    pub fn z(&self) -> f32 {
        unsafe { self.value[2] }
    }

    pub fn w(&self) -> f32 {
        unsafe { self.value[3] }
    }
    
    /// Ignores the `w` component when computing the cross product.
    pub fn cross(&self, rhs: Self) -> Self {
        // cross(a, b) = a.yzx * b.zxy - a.zxy * b.yzx.
        // cross(a, b) = (a * b.yzx - a.yzx * b).yzx.
        const MASK_YZX: i32 = 0b11001001;
        unsafe {
            let a_yzx = _mm_shuffle_ps::<MASK_YZX>(self.simd, self.simd);
            let b_yzx = _mm_shuffle_ps::<MASK_YZX>(rhs.simd, rhs.simd);
            let c = _mm_sub_ps(_mm_mul_ps(self.simd, b_yzx), _mm_mul_ps(rhs.simd, a_yzx));
            Self { simd: _mm_shuffle_ps::<MASK_YZX>(c, c) }
        }
    }
}

#[cfg(target_feature = "sse4.1")]
impl Vector4 {
    pub fn norm2(&self) -> f32 {
        unsafe { Self { simd: _mm_dp_ps::<0xff>(self.simd, self.simd) }.x() }
    }

    pub fn norm(&self) -> f32 {
        unsafe { f32::sqrt(Self { simd: _mm_dp_ps::<0xff>(self.simd, self.simd) }.x()) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { 
            let norm2_vec = _mm_dp_ps::<0xff>(self.simd, self.simd);
            Self { simd: _mm_mul_ps(self.simd, _mm_rsqrt_ps(norm2_vec)) } 
        }
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        unsafe { Self { simd: _mm_dp_ps::<0xff>(self.simd, rhs.simd) }.x() }
    }
}

#[cfg(not(target_feature = "sse4.1"))]
impl Vector4 {
    pub fn norm2(&self) -> f32 {
        self.dot(*self)
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.dot(*self))
    }

    pub fn normalize(&self) -> Self {
        *self / self.norm()
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        // This solution was adapted from Peter Cordes' answer on StackOverflow.
        // URL: https://stackoverflow.com/questions/6996764/fastest-way-to-do-horizontal-sse-vector-sum-or-other-reduction.
        unsafe {
            let product_vec = _mm_mul_ps(self.simd, rhs.simd);
            let mut shuf = _mm_shuffle_ps::<0b10110001>(product_vec, product_vec);
            let mut sum_vec = _mm_add_ps(product_vec, shuf);
            shuf = _mm_movehl_ps(shuf, sum_vec);
            sum_vec = _mm_add_ss(sum_vec, shuf);
            _mm_cvtss_f32(sum_vec)
        }
    }
}

impl convert::From<[f32; 4]> for Vector4 {
    fn from(value: [f32; 4]) -> Self {
        Self { value }
    }
}

impl ops::Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self { simd: _mm_add_ps(self.simd, rhs.simd) } }
    }
}

impl ops::Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self { simd: _mm_sub_ps(self.simd, rhs.simd) } }
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe { Self { simd: _mm_mul_ps(self.simd, _mm_set1_ps(rhs)) } }
    }
}

impl ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        unsafe { Vector4 { simd: _mm_mul_ps(_mm_set1_ps(self), rhs.simd) } }
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Vector4) -> Self::Output {
        unsafe { Vector4 { simd: _mm_mul_ps(self.simd, rhs.simd) } }
    }
}

impl ops::Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        unsafe { Vector4 { simd: _mm_div_ps(self.simd, _mm_set1_ps(rhs)) } }
    }
}

impl ops::Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl ops::AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<Vector4> for Vector4 {
    fn mul_assign(&mut self, rhs: Vector4) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm2() {
        let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v.norm2(), 1.0 + 1.0 + 1.0 + 1.0);
    }

    #[test]
    fn test_equality() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(v1, v1);
        assert_eq!(v2, v2);
        assert_ne!(v1, v2);
    }
}