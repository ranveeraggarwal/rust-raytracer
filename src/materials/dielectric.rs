extern crate rand;

use structures::vec3::Vec3;
use structures::ray::Ray;
use objects::HitRecord;

use super::Scatterable;
use super::metal::reflect;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ri: ri }
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

fn schlick(cosine: f64, ri: f64) -> f64 {
    let mut r0: f64 = (1.0 - ri)/(1.0 + ri);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosine).powi(5);
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(&ray_in.direction().unit_vector(), &rec.normal());
        let ni_over_nt: f64;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let reflect_prob: f64;
        let cosine: f64;
        if ray_in.direction().dot(&rec.normal()) > 0.0 {
            outward_normal = -1.0*rec.normal();
            ni_over_nt = self.ri;
            cosine = self.ri*ray_in.direction().dot(&rec.normal())/ray_in.direction().length();
        } else {
            outward_normal = 1.0*rec.normal();
            ni_over_nt = 1.0/self.ri;
            cosine = -1.0*ray_in.direction().dot(&rec.normal())/ray_in.direction().length();
        }
        if refract(&ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ri);
        } else {
            *scattered = Ray::new(rec.p(), reflected);
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            *scattered = Ray::new(rec.p(), reflected);
        } else {
            *scattered = Ray::new(rec.p(), refracted);
        }
        return true;
    }
}