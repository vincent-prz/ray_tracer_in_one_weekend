use super::vec3::dot;
use super::vec3::Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {albedo: Color},
    Metal { albedo: Color, fuzz: Option<f64> },
}


impl Material {
    pub fn new() -> Material {
        Material::Lambertian { albedo: Vec3(0.0, 0.0, 0.0) }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color,
         scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                Material::lambertian_scatter(albedo, hit_record, attenuation, scattered)
            }
            Material::Metal { albedo, fuzz } => {
                Material::metal_scatter(albedo, *fuzz, r_in, hit_record, attenuation, scattered)
            }
        }
    }

    fn lambertian_scatter(albedo: &Color, hit_record: &HitRecord,
        attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        *attenuation = *albedo;
        true
    }

    fn metal_scatter(albedo: &Color, fuzz: Option<f64>, r_in: &Ray, hit_record: &HitRecord,
        attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let v = r_in.direction;
        let n = hit_record.normal;
        let b = -dot(&v, &n) * n;
        let fuzz_value = match fuzz {
            Some(val) => val,
            None => 0.0
        };
        let scatter_direction = v + 2.0 * b + fuzz_value * Vec3::random_in_unit_sphere();
        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        *attenuation = *albedo;
        dot(&scatter_direction, &n) > 0.0
    }
}