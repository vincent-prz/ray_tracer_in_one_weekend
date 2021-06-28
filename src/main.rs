extern crate rand;
mod vec3;
mod camera;
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


pub fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> color::Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }
    let mut hit_record = hittable::HitRecord::new();
    let hit = world.hit(ray, 0.001, 100.0, &mut hit_record);
    if hit {
        // hacky diffuse rendered
        // let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere();
        // true lambertien renderer
        // let target = hit_record.p + hit_record.normal + Vec3::random_unit_vector();
        // alternative renderer (8.6)
        let target = hit_record.p + Vec3::random_in_hemisphere(&hit_record.normal);
        let ray = Ray {
            origin: hit_record.p,
            direction: target - hit_record.p,
        };
        return 0.5 * ray_color(&ray, world, depth - 1);
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
    let max_depth = 50;

    // World
    let mut world: HittableList<sphere::Sphere> = HittableList::new();
    world.add(sphere::Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.add(sphere::Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0});

    // Camera
    let cam = camera::Camera::new();

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
                let ray = cam.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth);
            }
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            color::write_color(stdout, color, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}
