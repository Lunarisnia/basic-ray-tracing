use crate::ray::Ray;
use crate::vector::Vector3;

pub struct HitRecord {
    pub position: Vector3,
    pub normal: Vector3,
    pub time: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            position: Vector3::zero(),
            normal: Vector3::zero(),
            time: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            Vector3(outward_normal.x(), outward_normal.y(), outward_normal.z())
        } else {
            -Vector3(outward_normal.x(), outward_normal.y(), outward_normal.z())
        }

    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, new_object: Box<dyn Hittable>) {
        self.objects.push(new_object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t_min: f32, ray_t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t_max;

        for object in &self.objects {
            if object.hit(ray, ray_t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.time;

                hit_record.position = temp_record.position;
                hit_record.normal = temp_record.normal;
                hit_record.time = temp_record.time;
                hit_record.front_face = temp_record.front_face;
            }
        }

        hit_anything
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f32, ray_t_max: f32, hit_record: &mut HitRecord) -> bool;
}