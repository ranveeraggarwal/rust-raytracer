use structures::ray::Ray;
use structures::vec3::Vec3;

use std::f64;

use self::sphere::Sphere;

pub mod sphere;
pub mod camera;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0)}
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }
}

pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let spheres_list: Vec<Sphere> = Vec::new();
        HittableList{spheres: spheres_list}
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for HittableList {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.size() {
            if self.spheres[i].intersect(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}

#[test]
fn test_hit_record() {
    let hit_record = HitRecord::new();
    assert_eq!(hit_record.normal(), Vec3::new(0.0, 0.0, 0.0));
}

#[test]
fn test_hittable() {
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let nx: u64 = 20;
    let ny: u64 = 10;

    let sphere1_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere_1: Sphere = Sphere::new(sphere1_center, 0.5);

    let mut world: HittableList = HittableList::new();
    world.add_sphere(sphere_1);

    let mut scene: Vec<Vec<Vec3>> = Vec::new();

    for y in (0..ny).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let u: f64 = x as f64/nx as f64;
            let v: f64 = y as f64/ny as f64;
            let r: Ray = Ray::new(origin,
                                  lower_left_corner + u*horizontal + v*vertical);
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            let mut rec: HitRecord = HitRecord::new();
            if world.intersect(&r, 0.0, f64::MAX, &mut rec) {
                color_vector = 255.99 * 0.5 * Vec3::new(rec.normal().x() + 1.0,
                                                        rec.normal().y() + 1.0,
                                                        rec.normal().z() + 1.0);
                color_vector.colorize();
            }
            row.push(color_vector);
        }
        scene.push(row);
    }

    assert!(true);
}