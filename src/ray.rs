use super::vec3::*;
use super::point3::Point3;
use super::color;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&ray.direction, &oc);
    let c = dot(&oc, &oc) - radius * radius;
    b * b - 4.0 * a * c > 0.0
}

pub fn ray_color(ray: &Ray) -> color::Color {
    if hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}
