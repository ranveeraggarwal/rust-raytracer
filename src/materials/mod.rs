use structures::ray::Ray;
use structures::vec3::Vec3;
use objects::HitRecord;

pub mod lambertian;
use self::lambertian::Lambertian;

pub enum Material {
    Lambertian,
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}