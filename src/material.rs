use super::vec3::dot;
use super::vec3::unit_vector;
use super::vec3::Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: Option<f64> },
    Dielectric { refraction_index: f64 },
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
            Material::Dielectric { refraction_index } => {
                Material::dielectric_scatter(*refraction_index, r_in, hit_record, attenuation, scattered)
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
        let v = unit_vector(r_in.direction);
        let n = hit_record.normal;
        let fuzz_value = match fuzz {
            Some(val) => val,
            None => 0.0
        };
        let scatter_direction = Vec3::reflect(v, n) + fuzz_value * Vec3::random_in_unit_sphere();
        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        *attenuation = *albedo;
        dot(scatter_direction, n) > 0.0
    }

    fn dielectric_scatter(refraction_index: f64, r_in: &Ray, hit_record: &HitRecord,
        attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let unit_dir = unit_vector(r_in.direction);
        let n = hit_record.normal;
        let refraction_ratio = if hit_record.front_face { 1.0 / refraction_index } else { refraction_index };
        let cos_theta = -dot(unit_dir, n);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let scatter_direction = if cannot_refract {
            Vec3::reflect(unit_dir, n)
        } else {
            Vec3::refract(unit_dir, n, refraction_ratio)
        };
        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        true
    }
}