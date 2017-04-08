use structures::vec3::Vec3;
use structures::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lower_left_corner: Vec3,
           horizontal: Vec3,
           vertical: Vec3,
           origin: Vec3) -> Camera {
        Camera{lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

#[test]
fn test_ray() {
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let camera: Camera = Camera::new(lower_left_corner, horizontal, vertical, origin);
    assert_eq!(camera.get_ray(0.0, 0.0), Ray::new(origin, lower_left_corner));
}