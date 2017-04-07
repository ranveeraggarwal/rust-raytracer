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

    pub fn intersects(ray: &Ray) -> bool {
        true
    }
}

#[test]
fn test_intersect() {
    let sphere_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(sphere_center, 0.5);
    assert!(true);
}