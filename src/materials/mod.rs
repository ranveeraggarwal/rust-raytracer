use structures::ray::Ray;
use structures::vec3::Vec3;
use objects::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

use self::lambertian::Lambertian;
use self::metal::Metal;
use self::dielectric::Dielectric;

#[derive(Clone, Copy, Debug)]
pub enum Material {
	Lambertian(Lambertian),
	Metal(Metal),
	Dielectric(Dielectric),
}

impl Scatterable for Material {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		match *self {
			Material::Lambertian(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
			Material::Metal(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
			Material::Dielectric(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
		}
	}
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}