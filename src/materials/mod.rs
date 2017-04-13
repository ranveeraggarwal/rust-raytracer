use structures::ray::Ray;
use structures::vec3::Vec3;
use objects::HitRecord;

pub mod lambertian;
pub mod metal;

use self::lambertian::Lambertian;
use self::metal::Metal;

#[derive(Clone, Copy, Debug)]
pub enum Material {
	Lambertian(Lambertian),
	Metal(Metal),
}

impl Scatterable for Material {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		match *self {
			Material::Lambertian(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
			Material::Metal(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
		}
	}
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}