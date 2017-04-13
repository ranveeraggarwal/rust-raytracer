use structures::vec3::Vec3;
use structures::ray::Ray;

use materials::Material;
#[cfg(test)]
use materials::lambertian::Lambertian;

use super::Hittable;
use super::HitRecord;

use std::f64;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {center: center, radius: radius, material: material}
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center();
        let a: f64 = ray.direction().dot(&ray.direction());
        let b: f64 = 2.0 * oc.dot(&ray.direction());
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t: f64 = (0.0 - b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center())/self.radius();
                rec.material = self.material;
                return true;
            }
            let t: f64 = (0.0 - b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center())/self.radius();
                rec.material = self.material;
                return true;
            }
        }
        false
    }
}

#[test]
fn test_intersect() {
    let sphere_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let lambert: Lambertian = Lambertian::new(Vec3::new(0.0, 0.0, 0.0));
    let sphere: Sphere = Sphere::new(sphere_center, 0.5, Material::Lambertian(lambert));
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let ray: Ray = Ray::new(origin, sphere_center);
    let mut rec: HitRecord = HitRecord::new();
    assert!(sphere.intersect(&ray, 0.0, f64::MAX, &mut rec));
}