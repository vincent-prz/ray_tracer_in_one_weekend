use crate::ray;
use crate::vec3;
use crate::point3;

pub struct HitRecord {
    pub p: point3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
}
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
