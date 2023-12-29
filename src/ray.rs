use crate::{interval::Interval, vector::Vec3};

use std::rc::Rc;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit>;
}

#[derive(Default)]
pub struct HittableList {
    inner: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.inner.push(object)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        let mut last_hit: Option<Hit> = None;

        for object in &self.inner {
            let max_t = last_hit.as_ref().map(|hit| hit.t).unwrap_or(ray_t.max);

            if let Some(hit) = object.hit(ray, ray_t.with_max(max_t)) {
                last_hit.replace(hit);
            }
        }

        last_hit
    }
}
