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

pub fn ray_color(ray: &Ray) -> color::Color {
    let sphere = sphere::Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    };
    // initialize the record with random values
    let mut hit_record = hittable::HitRecord {
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: 0.0
    };
    let hit = sphere.hit(ray, 0.0, 100.0, &mut hit_record);
    if hit {
        let normal = hit_record.normal;
        // 0.5(v + 1) -> [-1, 1] => [0, 1]
        return 0.5 * Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}
