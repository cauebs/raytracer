use std::io::{stdout, Write};

use crate::{vec3, FrameBuffer, Hittable, Interval, Ray, Vec3};

pub struct Camera {
    center: Vec3,
    focal_length: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
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
            center: vec3(0., 0., 0.),
            focal_length: 1.0,

            viewport_u,
            viewport_v,

            pixel_delta_u: viewport_u / output_width as f64,
            pixel_delta_v: viewport_v / output_height as f64,

            output_width,
            output_height,
        }
    }

    pub fn render(&self, world: &impl Hittable, fb: &mut FrameBuffer) {
        let viewport_upper_left = self.center
            - vec3(0., 0., self.focal_length)
            - self.viewport_u / 2.
            - self.viewport_v / 2.;

        let pixels_origin = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.;

        for y in 0..self.output_height {
            print!("\rScanlines remaining: {}  ", self.output_height - y);
            let _ = stdout().flush();

            for x in 0..self.output_width {
                let pixel_center = pixels_origin
                    + (self.pixel_delta_u * x as f64)
                    + (self.pixel_delta_v * y as f64);
                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);
                fb.paint(x, y, self.ray_color(&ray, world));
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable) -> Vec3 {
        if let Some(hit) = world.hit(ray, Interval::new(0., f64::INFINITY), &None) {
            return (hit.outward_normal + vec3(1., 1., 1.)) / 2.;
        }

        let unit_direction = ray.direction.normalize();
        let a = (unit_direction.y + 1.) / 2.;
        let white = vec3(1., 1., 1.);
        let blue = vec3(0.5, 0.7, 1.0);
        white * (1.0 - a) + blue * a
    }
}
