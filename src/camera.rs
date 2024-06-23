use Vector3 as Color;

use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::mathlib::random_zero_one;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector3;

pub struct Camera {
    image_width: i32,
    aspect_ratio: f32,
    samples_per_pixel: i32,
    pixel_sample_scale: f32,

    image_height: i32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f32, samples_per_pixel: i32) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            pixel_sample_scale: 0.0,
            image_height: 0,
            center: Vector3::zero(),
            pixel00_loc: Vector3::zero(),
            pixel_delta_u: Vector3::zero(),
            pixel_delta_v: Vector3::zero(),
        }
    }

    fn initialize(&mut self) {
        // Calculate image height ensure it's at least 1
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1
        }

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;


        // Arbitrary viewport height
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (self.image_width as f32 / self.image_height as f32);
        self.center = Vector3(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vector3 = Vector3(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3 = Vector3(0.0, -viewport_height, 0.0);

        // Calculate viewport
        let viewport_upper_left: Vector3 = &self.center - Vector3(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
        // Calculate distance between pixels
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        println!("P3");
        println!("{} {}", &self.image_width, &self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("Rendering.... {} Lines Remaining", &self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, &world);
                }
                self.write_color(self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("Rendering finished!");
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let kernel: Vector3 = Camera::sample_square();
        let pixel_sample: Vector3 = &self.pixel00_loc +
            ((i as f32 + kernel.x()) * &self.pixel_delta_u) +
            ((j as f32 + kernel.y()) * &self.pixel_delta_v);

        let ray_direction: Vector3 = pixel_sample - &self.center;

        Ray::new(&self.center, &ray_direction)
    }

    fn sample_square() -> Vector3 {
        Vector3(random_zero_one() - 0.5, random_zero_one() - 0.5, 0.0)
    }

    fn write_color(&self, color: Color) {
        let normalized_r: f32 = color.x();
        let normalized_g: f32 = color.y();
        let normalized_b: f32 = color.z();

        let interval: Interval = Interval::new(0.0, 0.999);
        let r: i32 = (255.999 * interval.clamp(normalized_r)) as i32;
        let g: i32 = (255.999 * interval.clamp(normalized_g)) as i32;
        let b: i32 = (255.999 * interval.clamp(normalized_b)) as i32;

        println!("{} {} {}", r, g, b);
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut hit_record: HitRecord = HitRecord::new();
        if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut hit_record) {
            let reflected_direction: Vector3 = Sphere::random_on_hemisphere(&hit_record.normal);
            return 0.5 * self.ray_color(&Ray::new(&hit_record.position, &reflected_direction), &world);
            // TODO: STOP HERE AND UNDERSTAND IT BEFORE PROCEEDING TO 9.2
        }

        let unit_direction: Vector3 = Vector3::unit_vector(ray.direction);
        let a: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Vector3(1.0, 1.0, 1.0) + a * Vector3(0.5, 0.7, 1.0)
    }
}