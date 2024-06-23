use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::vector::Vector3;

mod vector;
mod ray;
mod hittable;
mod sphere;
mod mathlib;
mod interval;
mod camera;


fn main() {
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(0.5, &Vector3(0.0, 0.0, -1.0))));
    world.add(Box::new(Sphere::new(100.0, &Vector3(0.0, -100.5, -1.0))));

    let image_width: i32 = 400;
    let aspect_ratio: f32 = 16.0 / 9.0;

    let mut camera: Camera = Camera::new(image_width, aspect_ratio, 100);
    camera.render(&world);
}
