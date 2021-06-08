use std::io;
use std::ops;

#[derive(Copy, Clone)]
struct Vec3(f64, f64, f64);

impl Vec3 {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 (
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 (
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 (
            self * v.x(),
            self * v.y(),
            self * v.z()
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 (
            self.x() / t,
            self.y() / t,
            self.z() / t
        )
    }
}


type Point3 = Vec3;
type Color = Vec3;

fn write_color(out: &mut Box<dyn io::Write>, pixel_color: Color) {
    let ir = (pixel_color.x() * 255.999) as u8;
    let ig = (pixel_color.y() * 255.999) as u8;
    let ib = (pixel_color.z() * 255.999) as u8;
    writeln!(out, "{} {} {}", ir, ig, ib);
}

fn unit_vector(v: &Vec3) -> Vec3 {
    (1.0 / v.length()) * (*v)
}

struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

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
            let ray = Ray { origin, direction };
            let color = ray_color(&ray);
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            write_color(stdout, color);
        }
    }
}
