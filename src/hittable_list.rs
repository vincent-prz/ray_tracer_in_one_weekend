use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::point3::Point3;

pub struct HittableList<T: Hittable> {
    objects: Vec<T>
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> HittableList<T> {
        HittableList { objects: Vec::new() }
    }
    pub fn add(&mut self, obj: T) {
        self.objects.push(obj);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}
