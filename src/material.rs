use euclid::approxeq::ApproxEq;

use crate::{
    ray::{self, Ray, ScatteredRay},
    vector::{random_unit_vector, Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &ray::Hit) -> Option<ScatteredRay>;
}

pub struct LambertianDiffuse {
    pub albedo: Color,
}

impl Material for LambertianDiffuse {
    fn scatter(&self, _ray: &Ray, hit: &ray::Hit) -> Option<ScatteredRay> {
        let mut direction = hit.normal + random_unit_vector();

        if direction.approx_eq(&Vec3::zero()) {
            direction = hit.normal;
        }

        Some(ScatteredRay {
            ray: Ray::new(hit.p, direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &ray::Hit) -> Option<ScatteredRay> {
        let direction = ray.direction.reflect(hit.normal) + random_unit_vector() * self.fuzziness;
        let above_surface = Vec3::dot(direction, hit.normal) > 0.;

        above_surface.then(|| ScatteredRay {
            ray: Ray::new(hit.p, direction),
            attenuation: self.albedo,
        })
    }
}
