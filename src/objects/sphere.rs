use structures::vec3::Vec3;
use structures::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {center: center, radius: radius}
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let oc: Vec3 = ray.origin() - self.center();
        let a: f64 = ray.direction().dot(&ray.direction());
        let b: f64 = 2.0 * oc.dot(&ray.direction());
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

#[test]
fn test_intersect() {
    let sphere_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(sphere_center, 0.5);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let ray: Ray = Ray::new(origin, sphere_center);
    assert!(sphere.intersects(&ray));
}