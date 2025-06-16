use std::{
    arch::x86_64::*,
    convert,
    fmt,
    ops
};

#[repr(C)]
#[derive(Clone, Copy)]
pub union Vector4 {
    simd: __m128,
    value: [f32; 4]
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
    
    #[allow(unreachable_code)]
    pub fn norm2(&self) -> f32 {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
        unsafe { return self.simd_norm2_sse41() }
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_norm2() }
        unsafe { self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2] + self.value[3] * self.value[3] }
    }

    #[allow(unreachable_code)]
    pub fn norm(&self) -> f32 {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
        unsafe { return self.simd_norm_sse41() }
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_norm() }
        unsafe { f32::sqrt(self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2] + self.value[3] * self.value[3]) }
    }

    #[allow(unreachable_code)]
    pub fn normalize(&self) -> Self {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
        unsafe { return self.simd_normalize_sse41() }
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_normalize() }
        *self / self.norm()
    }

    #[allow(unreachable_code)]
    pub fn dot(&self, rhs: Self) -> f32 {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
        unsafe { return self.simd_dot_sse41(rhs) }
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_dot(rhs) }
        unsafe { self.value[0] * rhs.value[0] + self.value[1] * rhs.value[1] + self.value[2] * rhs.value[2] + self.value[3] * rhs.value[3] }
    }

    /// Ignores the `w` component when computing the cross product.
    #[allow(unreachable_code)]
    pub fn cross(&self, rhs: Self) -> Self {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_cross(rhs) }
        unsafe { Self {
            value: [
                self.value[1] * rhs.value[2] - rhs.value[1] * self.value[2],
                rhs.value[0] * self.value[2] - self.value[0] * rhs.value[2],
                self.value[0] * rhs.value[1] - rhs.value[0] * self.value[1],
                0.0
            ]
        } }
    }
}

// Vector arithmetic using x86/x86_64 SSE intrinsics.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
impl Vector4 {
    #[target_feature(enable = "sse")]
    fn simd_eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp_vec = _mm_cmpeq_ps(self.simd, other.simd);
            _mm_movemask_ps(cmp_vec) == 0xf
        }
    }
    
    #[target_feature(enable = "sse")]
    fn simd_add(self, rhs: Self) -> Self {
        unsafe { Self { simd: _mm_add_ps(self.simd, rhs.simd) } }
    }

    #[target_feature(enable = "sse")]
    fn simd_sub(self, rhs: Self) -> Self {
        unsafe { Self { simd: _mm_sub_ps(self.simd, rhs.simd) } }
    }
    
    #[target_feature(enable = "sse")]
    fn simd_mul_scalar(self, rhs: f32) -> Self {
        unsafe { Self { simd: _mm_mul_ps(self.simd, _mm_set1_ps(rhs)) } }
    }

    #[target_feature(enable = "sse")]
    fn simd_mul_vec(self, rhs: Self) -> Self {
        unsafe { Self { simd: _mm_mul_ps(self.simd, rhs.simd) } }
    }

    #[target_feature(enable = "sse")]
    fn simd_div(self, rhs: f32) -> Self {
        unsafe { Vector4 { simd: _mm_div_ps(self.simd, _mm_set1_ps(rhs)) } }
    }
    
    /// Ignores the `w` component when computing the cross product.
    #[target_feature(enable = "sse")]
    fn simd_cross(&self, rhs: Self) -> Self {
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

    #[target_feature(enable = "sse")]
    fn simd_norm2(&self) -> f32 {
        self.simd_dot(*self)
    }

    #[target_feature(enable = "sse")]
    fn simd_norm(&self) -> f32 {
        f32::sqrt(self.simd_dot(*self))
    }

    #[target_feature(enable = "sse")]
    fn simd_normalize(&self) -> Self {
        unsafe {
            let norm2_vec = _mm_set1_ps(self.simd_dot(*self));
            Self { simd: _mm_mul_ps(self.simd, _mm_rsqrt_ps(norm2_vec)) } 
        }
    }

    #[target_feature(enable = "sse")]
    fn simd_dot(&self, rhs: Self) -> f32 {
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

// Optimised versions of functions based on dot products using SSE4.1 intrinsics.
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
impl Vector4 {
    #[target_feature(enable = "sse4.1")]
    fn simd_norm2_sse41(&self) -> f32 {
        unsafe { Self { simd: _mm_dp_ps::<0xff>(self.simd, self.simd) }.x() }
    }

    #[target_feature(enable = "sse4.1")]
    fn simd_norm_sse41(&self) -> f32 {
        unsafe { f32::sqrt(Self { simd: _mm_dp_ps::<0xff>(self.simd, self.simd) }.x()) }
    }

    #[target_feature(enable = "sse4.1")]
    fn simd_normalize_sse41(&self) -> Self {
        unsafe { 
            let norm2_vec = _mm_dp_ps::<0xff>(self.simd, self.simd);
            Self { simd: _mm_mul_ps(self.simd, _mm_rsqrt_ps(norm2_vec)) } 
        }
    }

    #[target_feature(enable = "sse4.1")]
    fn simd_dot_sse41(&self, rhs: Self) -> f32 {
        unsafe { Self { simd: _mm_dp_ps::<0xff>(self.simd, rhs.simd) }.x() }
    }
}

impl convert::From<[f32; 4]> for Vector4 {
    fn from(value: [f32; 4]) -> Self {
        Self { value }
    }
}

impl fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.value) }
    }
}

impl PartialEq for Vector4 {
    #[allow(unreachable_code)]
    fn eq(&self, other: &Self) -> bool {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_eq(other) }
        unsafe { self.value == other.value }
    }
}

impl ops::Add for Vector4 {
    type Output = Self;

    #[allow(unreachable_code)]
    fn add(self, rhs: Self) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_add(rhs) }
        unsafe { Self { value: [self.value[0] + rhs.value[0], self.value[1] + rhs.value[1], self.value[2] + rhs.value[2], self.value[3] + rhs.value[3]] } }
    }
}

impl ops::Sub for Vector4 {
    type Output = Self;

    #[allow(unreachable_code)]
    fn sub(self, rhs: Self) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_sub(rhs) }
        unsafe { Self { value: [self.value[0] - rhs.value[0], self.value[1] - rhs.value[1], self.value[2] - rhs.value[2], self.value[3] - rhs.value[3]] } }
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Self;

    #[allow(unreachable_code)]
    fn mul(self, rhs: f32) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_mul_scalar(rhs) }
        unsafe { Self { value: [self.value[0] * rhs, self.value[1] * rhs, self.value[2] * rhs, self.value[3] * rhs] } }
    }
}

impl ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    #[allow(unreachable_code)]
    fn mul(self, rhs: Vector4) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return rhs.simd_mul_scalar(self) }
        unsafe { Vector4 { value: [self * rhs.value[0], self * rhs.value[1], self * rhs.value[2], self * rhs.value[3]] } }
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = Self;

    #[allow(unreachable_code)]
    fn mul(self, rhs: Vector4) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_mul_vec(rhs) }
        unsafe { Self { value: [self.value[0] * rhs.value[0], self.value[1] * rhs.value[1], self.value[2] * rhs.value[2], self.value[3] * rhs.value[3]] } }
    }
}

impl ops::Div<f32> for Vector4 {
    type Output = Self;

    #[allow(unreachable_code)]
    fn div(self, rhs: f32) -> Self::Output {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse"))]
        unsafe { return self.simd_div(rhs) }
        unsafe { Self { value: [self.value[0] / rhs, self.value[1] / rhs, self.value[2] / rhs, self.value[3] / rhs] } }
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
    fn test_construct() {
        assert_eq!(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4 { value: [1.0, 2.0, 3.0, 4.0] });
    }

    #[test]
    fn test_from() {
        assert_eq!(Vector4::from([1.0, 2.0, 3.0, 4.0]), Vector4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let v3 = Vector4::new(1.5, -1.5, 0.6, 2.0);
        let v4 = Vector4::new(0.7, -8.0, -0.9, -2.0);
        assert_eq!(v1 + v2, v2);
        assert_eq!(v1 + v3, v3);
        assert_eq!(v1 + v4, v4);
        // assert_eq!(v2 + v3, Vector4::new(2.5, -0.5, 1.6));
        // assert_eq!(v2 + v4, Vector4::new(1.7, -7.0, 0.1));
        // assert_eq!(v3 + v4, Vector4::new(2.2, -9.5, -0.3));
        assert_eq!(v2 + v3, Vector4::new(1.0 + 1.5, 1.0 + (-1.5), 1.0 + 0.6, 1.0 + 2.0));
        assert_eq!(v2 + v4, Vector4::new(1.0 + 0.7, 1.0 + (-8.0), 1.0 + (-0.9), 1.0 + (-2.0)));
        assert_eq!(v3 + v4, Vector4::new(0.7 + 1.5, -8.0 + (-1.5), -0.9 + 0.6, 2.0 + (-2.0)));
        assert_eq!(v3 + v4, v4 + v3);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.9, 8.9, -0.65, 1.0);
        let v3 = Vector4::new(1.3, 0.37, 0.5, 1.0);
        let v4 = Vector4::new(0.9, 8.2, 0.21, 1.0);
        assert_eq!(v2 - v1, v2);
        assert_eq!(v4 - v1, v4);
        assert_eq!(v2 - v3, Vector4::new(0.9 - 1.3, 8.9 - 0.37, -0.65 - 0.5, 1.0 - 1.0));
        assert_eq!(v3 - v4, Vector4::new(1.3 - 0.9, 0.37 - 8.2, 0.5 - 0.21, 1.0 - 1.0));
        assert_eq!(v4 - v3, Vector4::new(0.9 - 1.3, 8.2 - 0.37, 0.21 - 0.5, 1.0 - 1.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vector4::new(0.821, -73.98, 0.98, 1.0);
        let v2 = Vector4::new(0.32, -9.821, 0.32, 1.0);
        let v3 = Vector4::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(0.932 * v1, v1 * 0.932);
        assert_eq!(-7.63 * v2, v2 * (-7.63));
        assert_eq!(8.0 * v3, v3 * 8.0);
        assert_eq!(0.0 * v2, Vector4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(v3 * 0.0, Vector4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(v1 * 0.76, Vector4::new(0.76 * 0.821, 0.76 * (-73.98), 0.76 * 0.98, 1.0 * 0.76));
        assert_eq!(1.8 * v2, Vector4::new(0.32 * 1.8, -9.821 * 1.8, 0.32 * 1.8, 1.0 * 1.8));
    }

    #[test]
    fn test_div() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.3, 0.4243, -0.9, 1.0);
        let v3 = Vector4::new(0.98, -312.0, 0.2, 2.0);
        let v4 = Vector4::new(-0.3, -0.1, 0.0, 3.0);
        assert_eq!(v1 / 0.6, v1);
        assert_eq!(v1 / 1.0, v1);
        assert_eq!(v2 / 1.63, Vector4::new(0.3 / 1.63, 0.4243 / 1.63, -0.9 / 1.63, 1.0 / 1.63));
        assert_eq!(v3 / (-0.987), Vector4::new(0.98 / (-0.987), -312.0 / (-0.987), 0.2 / (-0.987), 2.0 / (-0.987)));
        assert_eq!(v4 / 0.5, Vector4::new(-0.3 / 0.5, -0.1 / 0.5, 0.0, 3.0 / 0.5));
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        assert_eq!(Vector4::new(9.7, -8.2, -0.00382, 4.5) / 0.0, Vector4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_neg() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.7, -0.9, 0.8, -3.0);
        let v3 = Vector4::new(-1.9, 0.4, 8.3, 7.36);
        assert_eq!(-v1, v1);
        assert_eq!(-v2, Vector4::new(-0.7, 0.9, -0.8, 3.0));
        assert_eq!(-v3, Vector4::new(1.9, -0.4, -8.3, -7.36));
    }

    #[test]
    fn test_add_assign() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, -0.0);
        let v2 = Vector4::new(0.7, -0.9, 0.8, 5.675);
        let v3 = Vector4::new(-1.9, 0.4, 8.3, 212.231);
        let mut a = v1;
        a += v2;
        assert_eq!(a, v1 + v2);
        a += v3;
        assert_eq!(a, v1 + v2 + v3);
    }

    #[test]
    fn test_sub_assign() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.7, -0.9, 0.8, 1.6263);
        let v3 = Vector4::new(-1.9, 0.4, 8.3, -3.0);
        let mut a = v1;
        a -= v2;
        assert_eq!(a, v1 - v2);
        a -= v3;
        assert_eq!(a, v1 - v2 - v3);
    }

    #[test]
    fn test_mul_assign() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.7, -0.9, 0.8, 5.0);
        let v3 = Vector4::new(-1.9, 0.4, 8.3, -6.9);
        let mut a = v1;
        let mut b = v2;
        let mut c = v3;
        a *= 0.36;
        b *= -7.32;
        c *= 1.32;
        assert_eq!(a, 0.36 * v1);
        assert_eq!(b, -7.32 * v2);
        assert_eq!(c, 1.32 * v3);
    }

    #[test]
    fn test_div_assign() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.7, -0.9, 0.8, 69.420);
        let v3 = Vector4::new(-1.9, 0.4, 8.3, -1.89);
        let mut a = v1;
        let mut b = v2;
        let mut c = v3;
        a /= 0.36;
        b /= -7.32;
        c /= 1.32;
        assert_eq!(a, v1 / 0.36);
        assert_eq!(b, v2 / (-7.32));
        assert_eq!(c, v3 / 1.32);
    }

    #[test]
    #[should_panic]
    fn test_div_assign_by_zero() {
        let mut v = Vector4::new(98.32323, 0.9328, -32.123, 2.0);
        v /= 0.0;
        assert_eq!(v, Vector4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_norm2() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let v3 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1.norm2(), 0.0);
        assert_eq!(v2.norm2(), 4.0);
        assert_eq!(v3.norm2(), 1.0 + 4.0 + 9.0 + 16.0);
    }

    #[test]
    fn test_norm() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let v3 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1.norm(), 0.0);
        assert_eq!(v2.norm(), f32::sqrt(4.0));
        assert_eq!(v3.norm(), f32::sqrt(1.0 + 4.0 + 9.0 + 16.0));
    }

    #[test]
    fn test_normalize() {
        let v2 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let v3 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(Vector4::normalize(&v2), v2 / f32::sqrt(4.0));
        assert_eq!(Vector4::normalize(&v3), v3 / f32::sqrt(1.0 + 4.0 + 9.0 + 16.0));
    }

    #[test]
    #[should_panic]
    fn test_normalize_zero() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), Vector4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let v3 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v1.dot(v1), 0.0);
        assert_eq!(v2.dot(v2), v2.norm2());
        assert_eq!(v3.dot(v3), v3.norm2());
        assert_eq!(v1.dot(v3), 0.0);
        assert_eq!(v2.dot(v3), v3.dot(v2));
        assert_eq!(v2.dot(v3), 1.0 * 1.0 + 1.0 * 2.0 + 1.0 * 3.0 + 1.0 * 4.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let v3 = Vector4::new(0.0, 0.0, 1.0, 0.0);
        assert_eq!(v1.cross(v2), v3);
        assert_eq!(v2.cross(v3), v1);
        assert_eq!(v3.cross(v1), v2);
    }

    #[test]
    fn test_multiply_components() {
        assert_eq!(Vector4::from([0.0; 4]), Vector4::from([1.0, 2.0, 3.0, 4.0]) * Vector4::from([0.0; 4]));
        assert_eq!(Vector4::new(1.0, 2.0, 3.0, 4.0) * Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(1.0 * 1.0, 2.0 * 2.0, 3.0 * 3.0, 4.0 * 4.0));
        assert_eq!(Vector4::new(1.0, 2.0, 3.0, 4.0) * Vector4::from([3.0; 4]), 3.0 * Vector4::new(1.0, 2.0, 3.0, 4.0));
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