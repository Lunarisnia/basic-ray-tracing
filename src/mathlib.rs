use std::f32::consts::PI;

use rand::prelude::ThreadRng;
use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_zero_one() -> f32 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(min..max)
}