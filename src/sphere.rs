use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct Sphere {
    pub radius: f32,
    pub center: Vector3,
}

impl Sphere {
    pub fn new(radius: f32, center: &Vector3) -> Sphere {
        Sphere {
            radius,
            center: Vector3(center.x(), center.y(), center.z()),
        }
    }

    fn random_unit_in_sphere() -> Vector3 {
        loop {
            let rand: Vector3 = Vector3::random_range(-1.0, 1.0);
            if rand.squared() < 1.0 {
                return rand
            }
        }
    }

    fn random_unit_vector() -> Vector3 {
        Vector3::unit_vector(Self::random_unit_in_sphere())
    }

    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let on_unit_sphere: Vector3 = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let ray_center: Vector3 = self.center - ray.origin;
        let a: f32 = ray.direction.squared();
        let h: f32 = ray.direction.dot(&ray_center);
        let c: f32 = ray_center.squared() - self.radius * self.radius;
        let discriminant: f32 = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        };

        let sqrt_d: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root: f32 = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        hit_record.time = root;
        hit_record.position = ray.at(hit_record.time);
        let outward_normal: Vector3 = (hit_record.position - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        true
    }

}