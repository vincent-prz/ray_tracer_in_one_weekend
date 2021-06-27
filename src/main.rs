extern crate rand;
mod vec3;
mod color;
mod point3;
mod ray;
// FIXME: I don;t understand why I need to put hittable here
mod hittable;
mod hittable_list;
mod sphere;
mod utils;
use hittable::Hittable;
use hittable_list::HittableList;
use crate::ray::*;
use crate::vec3::*;
use std::io;


pub fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> color::Color {
    // initialize the record with random values
    let mut hit_record = hittable::HitRecord {
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: true,
    };
    let hit = world.hit(ray, 0.0, 100.0, &mut hit_record);
    if hit {
        let normal = hit_record.normal;
        // 0.5(v + 1) -> [-1, 1] => [0, 1]
        return 0.5 * Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let mut world: HittableList<sphere::Sphere> = HittableList::new();
    world.add(sphere::Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.add(sphere::Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0});

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("{} lines remaining", j + 1);
        for i in 0..image_width {
            let mut color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let random_val: f64 = rand::random();
                let u = (i as f64 + random_val) / (image_width - 1) as f64;
                let random_val: f64 = rand::random();
                let v = (j as f64 + random_val) / (image_height - 1) as f64;
                let direction = lower_left_corner + u * horizontal + v * vertical - origin;
                let ray = ray::Ray { origin, direction };
                color = color + ray_color(&ray, &world);
            }
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            color::write_color(stdout, color, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}
