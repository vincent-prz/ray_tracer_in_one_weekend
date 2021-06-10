use crate::vec3::Vec3;
use std::io;

pub type Color = Vec3;

pub fn write_color(out: &mut Box<dyn io::Write>, pixel_color: Color) {
    let ir = (pixel_color.x() * 255.999) as u8;
    let ig = (pixel_color.y() * 255.999) as u8;
    let ib = (pixel_color.z() * 255.999) as u8;
    writeln!(out, "{} {} {}", ir, ig, ib);
}
