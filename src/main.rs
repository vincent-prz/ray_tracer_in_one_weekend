use std::io;
use std::ops;

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

type Point3 = Vec3;
type Color = Vec3;

fn write_color(out: &mut Box<dyn io::Write>, pixel_color: Color) {
    let ir = (pixel_color.x() * 255.999) as u8;
    let ig = (pixel_color.y() * 255.999) as u8;
    let ib = (pixel_color.z() * 255.999) as u8;
    writeln!(out, "{} {} {}", ir, ig, ib);
}


fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let color = Vec3(r, g ,b);
            let stdout = &mut (Box::new(io::stdout()) as Box<dyn io::Write>);
            write_color(stdout, color);
        }
    }
}
