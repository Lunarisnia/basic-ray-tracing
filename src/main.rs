use Vector3 as Color;

use crate::ray::Ray;
use crate::vector::Vector3;

mod vector;
mod ray;

fn write_color(color: Color) {
    let normalized_r: f32 = color.x();
    let normalized_g: f32 = color.y();
    let normalized_b: f32 = color.z();

    let r: i32 = (255.999 * normalized_r) as i32;
    let g: i32 = (255.999 * normalized_g) as i32;
    let b: i32 = (255.999 * normalized_b) as i32;

    println!("{} {} {}", r, g, b);
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction: Vector3 = Vector3::unit_vector(ray.direction);
    let a: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color(1.0, 1.0, 1.0) + a * Color(0.5, 0.7, 1.0)
}


fn main() {
    let aspect_ratio: f32 = 16.0/9.0;
    let image_width: i32 = 400;

    // Calculate image height ensure it's at least 1
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1
    }

    // Arbitrary viewport height
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center: Vector3 = Vector3(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vector3 = Vector3(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3 = Vector3(0.0, -viewport_height, 0.0);

    // Calculate viewport
    let viewport_upper_left: Vector3 = &camera_center - Vector3(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    // Calculate distance between pixels
    let pixel_delta_u: Vector3 = viewport_u / image_width as f32;
    let pixel_delta_v: Vector3 = viewport_v / image_height as f32;

    let pixel00_loc: Vector3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{} {}", &image_width, &image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("Rendering.... {} Lines Remaining", &image_height - j);
        for i in 0..image_width {
            let pixel_center: Vector3 = &pixel00_loc + (i as f32 * &pixel_delta_u) + (j as f32 * &pixel_delta_v);
            let ray_direction: Vector3 = pixel_center - &camera_center;
            let ray: Ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color: Color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
    eprintln!("Rendering finished!");
}
