use crate::ray;
use crate::vec3;
use crate::point3;

pub struct HitRecord {
    pub p: point3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Vec3(0.0, 0.0, 0.0),
            normal: vec3::Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-(*outward_normal)};
    }
}
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
