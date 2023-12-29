use crate::{
    interval::Interval,
    ray::{self, Hittable, Ray},
    vector::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<ray::Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.square_length();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.square_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let root = (-half_b - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            let root = (-half_b + discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(ray::Hit {
            t,
            p,
            normal,
            front_face,
        })
    }
}
