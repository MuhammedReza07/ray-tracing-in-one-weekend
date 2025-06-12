use crate::{
    materials::Tangible,
    ray::Ray
};
use std::f64;

pub struct Intersection {
    pub t: f64,
    pub index: usize
}

pub struct RenderableList {
    elements: Vec<Box<dyn Tangible>>
}

impl RenderableList {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn get(&self, index: usize) -> &Box<dyn Tangible> {
        &self.elements[index]
    }

    pub fn push(&mut self, element: Box<dyn Tangible>) {
        self.elements.push(element);
    }

    /// Finds the smallest value of `t` such that `r` intersects an element of the list and `t` lies in `[t_min, t_max]`, and the index `i`
    /// 
    /// of the list element that yields the minimal `t`. Returns `Some(Intersection { t, i })` if such a `t` is found, `None` otherwise.
    pub fn intersect(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection_index: usize = 0;
        let mut intersection_t = f64::INFINITY;
        for (i, e) in self.elements.iter().enumerate() {
            if let Some(t) = e.intersect(r, t_min, t_max) {
                if t < intersection_t {
                    intersection_index = i;
                    intersection_t = t;
                }
            } else {
                continue;
            }
        }
        // intersection_t should never be f64::INFINITY.
        match intersection_t.is_finite() {
            true => Some(Intersection { t: intersection_t, index: intersection_index }),
            _ => None
        }
    }
}