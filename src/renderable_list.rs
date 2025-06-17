use crate::{
    materials::Tangible,
    ray::Ray
};
use rand::Rng;

pub struct RenderableList<R: Rng + ?Sized> {
    elements: Vec<Box<dyn Tangible<R> + Send + Sync>>
}

impl<R: Rng + ?Sized> RenderableList<R> {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn get(&self, index: usize) -> &(dyn Tangible<R> + Send + Sync) {
        &*self.elements[index]
    }

    pub fn push(&mut self, element: Box<dyn Tangible<R> + Send + Sync>) {
        self.elements.push(element);
    }

    /// Finds the smallest value of `t` such that `r` intersects an element of the list and `t` lies in `[t_min, t_max]`, and the index `i`
    /// 
    /// of the list element that yields the minimal `t`. Returns `Some(Intersection { t, i })` if such a `t` is found, `None` otherwise.
    pub fn intersect(&self, r: Ray, t_min: f32, t_max: f32) -> (f32, &(dyn Tangible<R> + Send + Sync)) {
        self.elements.iter()
        .map(|e| (e.intersect(r, t_min, t_max), &**e))
        .fold((f32::INFINITY, &*self.elements[0]), |acc, e| if e.0 < acc.0 { e } else { acc })
    }
}