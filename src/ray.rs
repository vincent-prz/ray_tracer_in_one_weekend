use super::vec3::*;
use super::point3::Point3;
use super::color;
use super::hittable;
use super::hittable::Hittable;
use super::sphere;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
