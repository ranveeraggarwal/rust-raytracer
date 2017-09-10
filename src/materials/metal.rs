use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::HitRecord;

use super::Scatterable;
use super::lambertian::point_in_unit_sphere;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let f: f64 = if fuzz < 1.0 {fuzz} else {1.0};
        Metal { albedo: albedo, fuzz: f }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = reflect(&ray_in.direction().unit_vector(), &rec.normal());
        *scattered = Ray::new(rec.p(), reflected + self.fuzz*point_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal()) > 0.0
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

#[test]
fn test_reflect() {
    let v: Vec3 = Vec3::new(1.0, 2.0, 4.0);
    let n: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    assert_eq!(reflect(&v, &n), v);
}