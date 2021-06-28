use super::vec3::Vec3;
use super::color::Color;
use super::ray::Ray;
use super::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {albedo: Color},
}


impl Material {
    pub fn new() -> Material {
        Material::Lambertian { albedo: Vec3(0.0, 0.0, 0.0) }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color,
         scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
                *scattered = Ray {
                    origin: hit_record.p,
                    direction: scatter_direction,
                };
                *attenuation = *albedo;
                true
            }
        }

    }
}