use crate::hittable;
use crate::ray;
use super::material::Material;
use crate::point3::Point3;
use super::vec3::dot;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Material,
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64,
           rec: &mut hittable::HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &oc);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / (2.0 * a);
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / (2.0 * a);
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.mat = self.mat;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        true
    }
} 
