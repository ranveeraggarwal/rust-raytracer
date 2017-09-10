use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::HitRecord;

use super::Scatterable;

use super::super::rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target: Vec3 = rec.p() + rec.normal() + point_in_unit_sphere();
        *scattered = Ray::new(rec.p(), target - rec.p());
        *attenuation = self.albedo;
        true
    }
}

pub fn point_in_unit_sphere() -> Vec3 {
    // Initialising p with a value outside the unit sphere
    let mut p: Vec3 = Vec3::new(2.0, 2.0, 2.0);
    let mut rng = super::super::rand::thread_rng();
    while p.dot(&p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

#[test]
fn test_point() {
    let vec: Vec3 = point_in_unit_sphere();
    assert_eq!(vec.dot(&Vec3::new(0.0, 0.0, 0.0)), 0.0);
}