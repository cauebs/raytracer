pub mod camera;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vector;

use camera::Camera;
use ray::HittableList;
use sphere::Sphere;
use vector::{vec3, Vec3};

use std::rc::Rc;

use anyhow::Result;
use minifb::{Key, Window, WindowOptions};

pub struct FrameBuffer {
    width: usize,
    buf: Vec<u32>,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            buf: vec![0; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn paint(&mut self, x: usize, y: usize, color: Vec3) {
        fn linear_to_gamma(component: f64) -> f64 {
            component.sqrt()
        }

        let i = self.index(x, y);
        self.buf[i] = u32::from_be_bytes([
            0,
            (linear_to_gamma(color.x) * 255.999) as u8,
            (linear_to_gamma(color.y) * 255.999) as u8,
            (linear_to_gamma(color.z) * 255.999) as u8,
        ]);
    }
}

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: usize = 500;
const HEIGHT: usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

fn main() -> Result<()> {
    let mut window = Window::new("Ray Tracing", WIDTH, HEIGHT, WindowOptions::default())?;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(vec3(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(vec3(0., -100.5, -1.), 100.)));

    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    let mut camera = Camera::new(WIDTH, HEIGHT)
        .with_samples_per_pixel(10)
        .with_max_bounces(5);

    camera.render(&world, &mut fb);
    window.update_with_buffer(&fb.buf, WIDTH, HEIGHT)?;

    while window.is_open() {
        let keys = window.get_keys();

        if keys.is_empty() {
            window.update();
            continue;
        }

        let move_speed = 0.05;
        for key in keys {
            match key {
                Key::NumPadPlus => {
                    camera.samples_per_pixel = camera.samples_per_pixel.saturating_add(50)
                }
                Key::NumPadMinus => {
                    camera.samples_per_pixel = camera.samples_per_pixel.saturating_sub(50)
                }
                Key::W => camera.center += vec3(0., 0., -move_speed),
                Key::A => camera.center += vec3(-move_speed, 0., 0.),
                Key::S => camera.center += vec3(0., 0., move_speed),
                Key::D => camera.center += vec3(move_speed, 0., 0.),
                Key::R => camera.center += vec3(0., move_speed, 0.),
                Key::F => camera.center += vec3(0., -move_speed, 0.),
                _ => {}
            }
        }

        camera.render(&world, &mut fb);
        window.update_with_buffer(&fb.buf, WIDTH, HEIGHT)?;
    }

    Ok(())
}
