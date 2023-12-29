pub mod camera;
pub mod interval;
pub mod ray;
pub mod sphere;

use camera::Camera;
pub use interval::Interval;
pub use ray::{Hittable, HittableList, Ray};
pub use sphere::Sphere;

use std::{rc::Rc, time::Duration};

use anyhow::Result;
pub use euclid::vec3;
use minifb::{Window, WindowOptions};

pub type Vec3 = euclid::default::Vector3D<f64>;

pub struct FrameBuffer {
    width: usize,
    // height: usize,
    buf: Vec<u32>,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            // height,
            buf: vec![0; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn paint(&mut self, x: usize, y: usize, color: Vec3) {
        let i = self.index(x, y);
        self.buf[i] = u32::from_be_bytes([
            0,
            (color.x * 255.999) as u8,
            (color.y * 255.999) as u8,
            (color.z * 255.999) as u8,
        ]);
    }
}

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: usize = 500;
const HEIGHT: usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

const MAX_FPS: f32 = 60.;

fn main() -> Result<()> {
    let mut window = Window::new("Ray Tracing", WIDTH, HEIGHT, WindowOptions::default())?;

    let frame_budget = Duration::from_secs_f32(1. / MAX_FPS);
    window.limit_update_rate(Some(frame_budget));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(vec3(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(vec3(0., -100.5, -1.), 100.)));

    let camera = Camera::new(WIDTH, HEIGHT);
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);

    while window.is_open() {
        camera.render(&world, &mut fb);
        window.update_with_buffer(&fb.buf, WIDTH, HEIGHT)?;
    }

    Ok(())
}
