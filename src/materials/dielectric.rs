use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::HitRecord;

use super::Scatterable;
use super::metal::reflect;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    albedo: Vec3,
    ri: f64,
}

impl Dielectric {
    pub fn new(albedo: Vec3, ri: f64) -> Dielectric {
        Dielectric { albedo: albedo, ri: ri }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv: Vec3 = v.unit_vector();
    let dt: f64 = uv.dot(&n);

    let discriminant: f64 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt*(*v - *n*dt) - *n*((discriminant).sqrt());
        return true;
    } else {
        return false;
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut outward_normal: Vec3;
        let reflected = reflect(&ray_in.direction().unit_vector(), &rec.normal());
        let mut ni_over_nt: f64;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3;
        if (ray_in.direction().dot(&rec.normal()) > 0.0) {
            outward_normal = -1.0*rec.normal();
            ni_over_nt = self.ri;
        } else {
            outward_normal = 1.0*rec.normal();
            ni_over_nt = 1.0/self.ri;
        }
        
        return true;
    }
}