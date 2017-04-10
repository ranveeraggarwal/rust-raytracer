extern crate raytracer;
extern crate rand;

use std::f64;

use raytracer::structures::vec3::Vec3;
use raytracer::structures::ray::Ray;

use raytracer::objects::sphere::Sphere;
use raytracer::objects::camera::Camera;
use raytracer::objects::{Hittable, HittableList, HitRecord};

use raytracer::io::write::gen_ppm;

use rand::Rng;

fn point_in_unit_sphere() -> Vec3 {
    // Initialising p with a value outside the unit sphere
    let mut p: Vec3 = Vec3::new(2.0, 2.0, 2.0);
    let mut rng = rand::thread_rng();
    while p.dot(&p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color (r: &Ray, world: &Hittable, depth: u64) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.intersect(&r, 0.0, f64::MAX, &mut rec) && depth > 0 {
        let target: Vec3 = rec.p() + rec.normal() + point_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p(), target - rec.p()), world, depth-1);
    } else {
        return Vec3::new(0.5, 0.5, 0.5);
    }
}

fn main() {
    let filename = "outputs/basic_sphere.ppm".to_string();

    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let nx: u64 = 200;
    let ny: u64 = 100;
    let ns: u64 = 100;

    let cam: Camera = Camera::new(lower_left_corner, horizontal, vertical, origin);

    let sphere1_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere_1: Sphere = Sphere::new(sphere1_center, 0.5);

    let sphere2_center: Vec3 = Vec3::new(0.0, -100.5, -1.0);
    let sphere_2: Sphere = Sphere::new(sphere2_center, 100.0);

    let mut world: HittableList = HittableList::new();
    world.add_sphere(sphere_1);
    world.add_sphere(sphere_2);

    let mut scene: Vec<Vec<Vec3>> = Vec::new();

    let mut rng = rand::thread_rng();

    for y in (0..ny).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (x as f64 + rng.gen::<f64>()) / nx as f64;
                let v: f64 = (y as f64 + rng.gen::<f64>()) / ny as f64;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + color(&r, &world, 3);
            }
            color_vector = color_vector/ns as f64;
            color_vector = 255.99*Vec3::new(color_vector.r().sqrt(), color_vector.g().sqrt(), color_vector.b().sqrt());
            color_vector.colorize();
            row.push(color_vector);
        }
        scene.push(row);
    }
    gen_ppm(scene, filename);
}