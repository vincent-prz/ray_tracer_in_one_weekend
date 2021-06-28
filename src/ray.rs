use super::vec3::*;
use super::point3::Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vec3(0.0, 0.0, 0.0),
            direction: Vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
