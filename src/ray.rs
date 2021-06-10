use super::vec3::Vec3;
use super::vec3::unit_vector;
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

pub fn ray_color(ray: &Ray) -> color::Color {
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}
