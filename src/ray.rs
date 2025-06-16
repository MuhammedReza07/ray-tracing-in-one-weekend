use crate::vector4::Vector4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector4,
    pub direction: Vector4
}

impl Ray {
    pub fn new(origin: Vector4, direction: Vector4) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector4 {
        self.origin + t * self.direction
    }

    pub fn length(&self, t: f32) -> f32 {
        (self.at(t) - self.origin).norm()
    }
}