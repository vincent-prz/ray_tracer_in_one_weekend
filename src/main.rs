mod vec3;
mod color;
mod point3;
mod ray;
use crate::vec3::Vec3;
use std::io;


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

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
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = ray::Ray { origin, direction };
            let color = ray::ray_color(&ray);
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            color::write_color(stdout, color);
        }
    }
}
