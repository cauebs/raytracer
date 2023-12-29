use indicatif::ProgressIterator;
use rand::random;

use crate::{
    interval::Interval,
    ray::{Hittable, Ray},
    vector::{self, vec3, Vec3},
    FrameBuffer,
};

pub struct Camera {
    pub center: Vec3,
    focal_length: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    output_width: usize,
    output_height: usize,
}

impl Camera {
    pub fn new(output_width: usize, output_height: usize) -> Self {
        let aspect_ratio = output_width as f64 / output_height as f64;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = vec3(viewport_width, 0., 0.);
        let viewport_v = vec3(0., -viewport_height, 0.);

        Self {
            center: Vec3::zero(),
            focal_length: 1.0,

            viewport_u,
            viewport_v,

            pixel_delta_u: viewport_u / output_width as f64,
            pixel_delta_v: viewport_v / output_height as f64,
            samples_per_pixel: 1,
            max_bounces: 1,

            output_width,
            output_height,
        }
    }

    pub fn with_samples_per_pixel(mut self, samples: usize) -> Self {
        self.samples_per_pixel = samples;
        self
    }

    pub fn with_max_bounces(mut self, bounces: usize) -> Self {
        self.max_bounces = bounces;
        self
    }

    pub fn render(&self, world: &impl Hittable, fb: &mut FrameBuffer) {
        println!(
            "Rendering with {} samples per pixel and {} max ray bounces...",
            self.samples_per_pixel, self.max_bounces
        );
        for y in (0..self.output_height).progress() {
            for x in 0..self.output_width {
                let mut pixel_color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.make_ray(x, y);
                    pixel_color += ray_color(&ray, world, self.max_bounces);
                }
                pixel_color /= self.samples_per_pixel as f64;
                fb.paint(x, y, pixel_color);
            }
        }
    }

    fn pixels_origin(&self) -> Vec3 {
        let viewport_upper_left = self.center
            - vec3(0., 0., self.focal_length)
            - self.viewport_u / 2.
            - self.viewport_v / 2.;

        viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.
    }

    fn pixel_sample_offset(&self) -> Vec3 {
        let px = random::<f64>() - 0.5;
        let py = random::<f64>() - 0.5;
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn make_ray(&self, window_x: usize, window_y: usize) -> Ray {
        let pixel_center = self.pixels_origin()
            + (self.pixel_delta_u * window_x as f64)
            + (self.pixel_delta_v * window_y as f64);

        let pixel_sample = pixel_center + self.pixel_sample_offset();

        let origin = self.center;
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }
}

fn ray_color(ray: &Ray, world: &impl Hittable, remaining_bounces: usize) -> Vec3 {
    if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        if remaining_bounces == 0 {
            return Vec3::zero();
        }
        let bounce_direction = vector::random_on_hemisphere(hit.normal);
        return ray_color(
            &Ray::new(hit.p, bounce_direction),
            world,
            remaining_bounces - 1,
        ) / 2.;
    }

    let unit_direction = ray.direction.normalize();
    let a = (unit_direction.y + 1.) / 2.;
    let white = Vec3::one();
    let blue = vec3(0.5, 0.7, 1.0);
    white * (1.0 - a) + blue * a
}
