use std::{convert, f64, ops};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    value: [f64; 3]
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { value: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.value[0]
    }

    pub fn y(&self) -> f64 {
        self.value[1]
    }

    pub fn z(&self) -> f64 {
        self.value[2]
    }

    pub fn norm2(&self) -> f64 {
        self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2]
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2])
    }
}

impl convert::From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Self { value }
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { value: [self.value[0] + rhs.value[0], self.value[1] + rhs.value[1], self.value[2] + rhs.value[2]] }
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { value: [self.value[0] - rhs.value[0], self.value[1] - rhs.value[1], self.value[2] - rhs.value[2]] }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: [self.value[0] * rhs, self.value[1] * rhs, self.value[2] * rhs] }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 { value: [self * rhs.value[0], self * rhs.value[1], self * rhs.value[2]] }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { value: [self.value[0] / rhs, self.value[1] / rhs, self.value[2] / rhs] }
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: [-self.value[0], -self.value[1], -self.value[2]] }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

pub fn dot(a: Vector3, b: Vector3) -> f64 {
    a.value[0] * b.value[0] + a.value[1] * b.value[1] + a.value[2] * b.value[2]
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    Vector3 {
        value: [
            a.value[1] * b.value[2] - b.value[1] * a.value[2],
            b.value[0] * a.value[2] - a.value[0] * b.value[2],
            a.value[0] * b.value[1] - b.value[0] * a.value[1]
        ]
    }
}

pub fn normalize(v: Vector3) -> Vector3 {
    v / v.norm()
}

#[cfg(test)]
mod tests {
    use std::f64;
    use super::*;

    #[test]
    fn test_construct() {
        assert_eq!(Vector3::new(1.0, 2.0, 3.0), Vector3 { value: [1.0, 2.0, 3.0] });
    }

    #[test]
    fn test_from() {
        assert_eq!(Vector3::from([1.0, 2.0, 3.0]), Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        let v3 = Vector3::new(1.5, -1.5, 0.6);
        let v4 = Vector3::new(0.7, -8.0, -0.9);
        assert_eq!(v1 + v2, v2);
        assert_eq!(v1 + v3, v3);
        assert_eq!(v1 + v4, v4);
        // assert_eq!(v2 + v3, Vector3::new(2.5, -0.5, 1.6));
        // assert_eq!(v2 + v4, Vector3::new(1.7, -7.0, 0.1));
        // assert_eq!(v3 + v4, Vector3::new(2.2, -9.5, -0.3));
        assert_eq!(v2 + v3, Vector3::new(1.0 + 1.5, 1.0 + (-1.5), 1.0 + 0.6));
        assert_eq!(v2 + v4, Vector3::new(1.0 + 0.7, 1.0 + (-8.0), 1.0 + (-0.9)));
        assert_eq!(v3 + v4, Vector3::new(0.7 + 1.5, -8.0 + (-1.5), -0.9 + 0.6));
        assert_eq!(v3 + v4, v4 + v3);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.9, 8.9, -0.65);
        let v3 = Vector3::new(1.3, 0.37, 0.5);
        let v4 = Vector3::new(0.9, 8.2, 0.21);
        assert_eq!(v2 - v1, v2);
        assert_eq!(v4 - v1, v4);
        assert_eq!(v2 - v3, Vector3::new(0.9 - 1.3, 8.9 - 0.37, -0.65 - 0.5));
        assert_eq!(v3 - v4, Vector3::new(1.3 - 0.9, 0.37 - 8.2, 0.5 - 0.21));
        assert_eq!(v4 - v3, Vector3::new(0.9 - 1.3, 8.2 - 0.37, 0.21 - 0.5));
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3::new(0.821, -73.98, 0.98);
        let v2 = Vector3::new(0.32, -9.821, 0.32);
        let v3 = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(0.932 * v1, v1 * 0.932);
        assert_eq!(-7.63 * v2, v2 * (-7.63));
        assert_eq!(8.0 * v3, v3 * 8.0);
        assert_eq!(0.0 * v2, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(v3 * 0.0, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(v1 * 0.76, Vector3::new(0.76 * 0.821, 0.76 * (-73.98), 0.76 * 0.98));
        assert_eq!(1.8 * v2, Vector3::new(0.32 * 1.8, -9.821 * 1.8, 0.32 * 1.8));
    }

    #[test]
    fn test_div() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.3, 0.4243, -0.9);
        let v3 = Vector3::new(0.98, -312.0, 0.2);
        let v4 = Vector3::new(-0.3, -0.1, 0.0);
        assert_eq!(v1 / 0.6, v1);
        assert_eq!(v1 / 1.0, v1);
        assert_eq!(v2 / 1.63, Vector3::new(0.3 / 1.63, 0.4243 / 1.63, -0.9 / 1.63));
        assert_eq!(v3 / (-0.987), Vector3::new(0.98 / (-0.987), -312.0 / (-0.987), 0.2 / (-0.987)));
        assert_eq!(v4 / 0.5, Vector3::new(-0.3 / 0.5, -0.1 / 0.5, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        assert_eq!(Vector3::new(9.7, -8.2, -0.00382) / 0.0, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_neg() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.7, -0.9, 0.8);
        let v3 = Vector3::new(-1.9, 0.4, 8.3);
        assert_eq!(-v1, v1);
        assert_eq!(-v2, Vector3::new(-0.7, 0.9, -0.8));
        assert_eq!(-v3, Vector3::new(1.9, -0.4, -8.3));
    }

    #[test]
    fn test_add_assign() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.7, -0.9, 0.8);
        let v3 = Vector3::new(-1.9, 0.4, 8.3);
        let mut a = v1;
        a += v2;
        assert_eq!(a, v1 + v2);
        a += v3;
        assert_eq!(a, v1 + v2 + v3);
    }

    #[test]
    fn test_sub_assign() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.7, -0.9, 0.8);
        let v3 = Vector3::new(-1.9, 0.4, 8.3);
        let mut a = v1;
        a -= v2;
        assert_eq!(a, v1 - v2);
        a -= v3;
        assert_eq!(a, v1 - v2 - v3);
    }

    #[test]
    fn test_mul_assign() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.7, -0.9, 0.8);
        let v3 = Vector3::new(-1.9, 0.4, 8.3);
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
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(0.7, -0.9, 0.8);
        let v3 = Vector3::new(-1.9, 0.4, 8.3);
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
        let mut v = Vector3::new(98.32323, 0.9328, -32.123);
        v /= 0.0;
        assert_eq!(v, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_norm2() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        let v3 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.norm2(), 0.0);
        assert_eq!(v2.norm2(), 3.0);
        assert_eq!(v3.norm2(), 1.0 + 4.0 + 9.0);
    }

    #[test]
    fn test_norm() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        let v3 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.norm(), 0.0);
        assert_eq!(v2.norm(), f64::sqrt(3.0));
        assert_eq!(v3.norm(), f64::sqrt(1.0 + 4.0 + 9.0));
    }

    #[test]
    fn test_normalize() {
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        let v3 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(normalize(v2), v2 / f64::sqrt(3.0));
        assert_eq!(normalize(v3), v3 / f64::sqrt(1.0 + 4.0 + 9.0));
    }

    #[test]
    #[should_panic]
    fn test_normalize_zero() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(normalize(v1), Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 1.0, 1.0);
        let v3 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(dot(v1, v1), 0.0);
        assert_eq!(dot(v2, v2), v2.norm2());
        assert_eq!(dot(v3, v3), v3.norm2());
        assert_eq!(dot(v1, v3), 0.0);
        assert_eq!(dot(v2, v3), dot(v3, v2));
        assert_eq!(dot(v2, v3), 1.0 * 1.0 + 1.0 * 2.0 + 1.0 * 3.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(cross(v1, v2), v3);
        assert_eq!(cross(v2, v3), v1);
        assert_eq!(cross(v3, v1), v2);
    }
}