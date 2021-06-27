use crate::vec3::Vec3;
use crate::utils;
use std::io;

pub type Color = Vec3;

pub fn write_color(out: &mut Box<dyn io::Write>, pixel_color: Color, samples_per_pixel: u32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();
    let scale = 1.0 / samples_per_pixel as f64;

    let ir = (utils::clamp(r * scale, 0.0, 0.999) * 256.0) as u8;
    let ig = (utils::clamp(g * scale, 0.0, 0.999) * 256.0) as u8;
    let ib = (utils::clamp(b * scale, 0.0, 0.999) * 256.0) as u8;
    writeln!(out, "{} {} {}", ir, ig, ib);
}
