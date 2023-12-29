use crate::{Interval, Vec3};

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
    pub outward_normal: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, previous_hit: &Option<Hit>) -> Option<Hit>;
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
    fn hit(&self, ray: &Ray, ray_t: Interval, previous_hit: &Option<Hit>) -> Option<Hit> {
        let mut last_hit = previous_hit.clone();
        let mut closest_so_far = ray_t.max;

        for object in &self.inner {
            if let Some(hit) = object.hit(ray, ray_t.with_max(closest_so_far), &last_hit) {
                closest_so_far = hit.t;
                last_hit.replace(hit);
            }
        }

        last_hit
    }
}
