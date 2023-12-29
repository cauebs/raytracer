pub use euclid::vec3;
use rand::Rng;

pub type Vec3 = euclid::default::Vector3D<f64>;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let mut r = || rng.gen_range(-1.0..1.0);
        let x = r();
        let y = r();
        let z = r();
        let p = vec3(x, y, z);
        if p.square_length() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if Vec3::dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
