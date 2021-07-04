extern crate rand;
mod vec3;
mod camera;
mod color;
mod material;
mod point3;
mod ray;
// FIXME: I don;t understand why I need to put hittable here
mod hittable;
mod hittable_list;
mod sphere;
mod utils;
use hittable::Hittable;
use hittable_list::HittableList;
use material::Material;
use crate::ray::*;
use crate::vec3::*;
use crate::sphere::Sphere;
use rand::random;
use std::io;
use std::time::Instant;


pub fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: u32) -> color::Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }
    let mut hit_record = hittable::HitRecord::new();
    let hit = world.hit(ray, 0.001, 1000.0, &mut hit_record);
    if hit {
        let mut scattered = Ray::new();
        let mut attenuation = Vec3(0.0, 0.0, 0.0);
        if hit_record.mat.scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let material_ground = Material::Lambertian { albedo: Vec3(0.5, 0.5, 0.5) };
    world.add(Box::new(Sphere {center: Vec3(0.0, -1000.0, 0.0), radius: 1000.0,
         mat: material_ground}));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let a_random: f64 = random();
            let b_random: f64 = random();
            let center = Vec3((a as f64) + 0.9 * a_random, 0.2, (b as f64) + 0.9 * b_random);
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let sphere_material = Material::Lambertian { albedo };
                    world.add(Box::new(Sphere { center, radius: 0.2, mat: sphere_material}))
                }  else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz : f64 = random();
                    let sphere_material = Material::Metal { albedo, fuzz: Some(fuzz) };
                    world.add(Box::new(Sphere { center, radius: 0.2, mat: sphere_material}))
                } else {
                    // glass
                    let sphere_material = Material::Dielectric { refraction_index: 1.5 };
                    world.add(Box::new(Sphere { center, radius: 0.2, mat: sphere_material}))
                }
            }
        }
    }
    let material_dielectric = Material::Dielectric { refraction_index: 1.5 };
    world.add(Box::new(Sphere { center: Vec3(0.0, 1.0, 0.0), radius: 1.0, mat: material_dielectric }));
    let material_lambertian = Material::Lambertian { albedo: Vec3(0.4, 0.2, 0.1) };
    world.add(Box::new(Sphere { center: Vec3(-4.0, 1.0, 0.0), radius: 1.0, mat: material_lambertian }));
    let material_metal = Material::Metal { albedo: Vec3(0.7, 0.6, 0.5), fuzz: None };
    world.add(Box::new(Sphere { center: Vec3(4.0, 1.0, 0.0), radius: 1.0, mat: material_metal }));
    world
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vertical_fov = 20.0; // vertical field of view

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let cam = camera::Camera::new(
        look_from,
        look_at,
        vup,
        vertical_fov,
        aspect_ratio
    );

    // render
    let render_instant = Instant::now();
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("{} lines remaining", j + 1);
        for i in 0..image_width {
            let mut color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let random_val: f64 = random();
                let u = (i as f64 + random_val) / (image_width - 1) as f64;
                let random_val: f64 = random();
                let v = (j as f64 + random_val) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth);
            }
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            color::write_color(stdout, color, samples_per_pixel);
        }
    }
    eprintln!("Done.");
    eprintln!("rendering duration: {}s", render_instant.elapsed().as_secs());
}
