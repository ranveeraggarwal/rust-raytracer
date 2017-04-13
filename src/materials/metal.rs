use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::HitRecord;

use super::Scatterable;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo: albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = reflect(&ray_in.direction().unit_vector(), &rec.normal());
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal()) > 0.0
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

#[test]
fn test_reflect() {
    let v: Vec3 = Vec3::new(1.0, 2.0, 4.0);
    let n: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    assert_eq!(reflect(&v, &n), v);
}