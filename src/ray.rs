use crate::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Ray {
        Ray{
            origin: Vector3(origin.x(), origin.y(), origin.z()),
            direction: Vector3(direction.x(), direction.y(), direction.z())
        }
    }
    pub fn at(&self, t: f32) -> Vector3 {
        let a: Vector3 = Vector3(self.origin.x(), self.origin.y(), self.origin.z());
        let b: Vector3 = Vector3(self.direction.x(), self.direction.y(), self.direction.z());
        a + t*b
    }
}