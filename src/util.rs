use rand::prelude::*;
use std::io::Write;
use crate::vec3::Color;


pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn deg2rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color<T>(stream: &mut T, pixel_color: Color,
                      samples_per_pixel: i32) where T: Write {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    write!(stream, "{} {} {}\n", ir, ig, ib).unwrap();
}

pub fn random_number() -> f32 {
    let mut rng = thread_rng();
    let the_number: f32 = rng.gen();
    the_number - 0.0001
}

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_number()
}
