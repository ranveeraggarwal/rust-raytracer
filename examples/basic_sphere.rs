extern crate raytracer;
extern crate rand;
extern crate rayon;
extern crate indicatif;

use std::f64;

use raytracer::structures::vec3::Vec3;
use raytracer::structures::ray::Ray;

use raytracer::materials::Material;
use raytracer::materials::Scatterable;
use raytracer::materials::lambertian::Lambertian;
use raytracer::materials::metal::Metal;
use raytracer::materials::dielectric::Dielectric;

use raytracer::objects::sphere::Sphere;
use raytracer::objects::camera::Camera;
use raytracer::objects::{Hittable, HittableList, HitRecord};

use raytracer::io::write::gen_ppm;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use indicatif::{ProgressBar, ProgressStyle};

fn color (r: &Ray, world: &Hittable, depth: u64) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.intersect(&r, 0.0001, f64::MAX, &mut rec)  {
        let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if depth > 0 && rec.material().scatter(r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth-1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction: Vec3 = r.direction().unit_vector();
        let t: f64 = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
        // return Vec3::new(0.5, 0.5, 0.5);
    }
}

fn main() {
    let filename = "outputs/basic_sphere.png".to_string();

    let nx: u64 = 400;
    let ny: u64 = 200;
    let ns: u64 = 200;

    let cam: Camera = Camera::new(Vec3::new(0.0, 1.0, 5.0), Vec3::new(0.0, 0.0, -1.0), 
    Vec3::new(0.0, 1.0, 0.0), 50.0, (nx as f64)/(ny as f64), 0.001, Vec3::new(3.0, 3.0, 3.0).length());

    let lambert_1: Lambertian = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let sphere1_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere_1: Sphere = Sphere::new(sphere1_center, 0.5, Material::Lambertian(lambert_1));

    let lambert_2: Lambertian = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let sphere2_center: Vec3 = Vec3::new(0.0, -100.5, -1.0);
    let sphere_2: Sphere = Sphere::new(sphere2_center, 100.0, Material::Lambertian(lambert_2));

    let metal_1: Metal = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);
    let sphere3_center: Vec3 = Vec3::new(1.0, -0.0, -1.0);
    let sphere_3: Sphere = Sphere::new(sphere3_center, 0.5, Material::Metal(metal_1));

    let die_1: Dielectric = Dielectric::new(1.5);
    let sphere4_center: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    let sphere_4: Sphere = Sphere::new(sphere4_center, 0.75, Material::Dielectric(die_1));

    let mut world: HittableList = HittableList::new();
    world.add_sphere(sphere_1);
    world.add_sphere(sphere_2);
    world.add_sphere(sphere_3);
    world.add_sphere(sphere_4);

    let bar = ProgressBar::new(ny);
    bar.set_style(ProgressStyle::default_bar().template("[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]"));

    let scene: Vec<Vec<Vec3>> = (0..ny).into_par_iter().map(|y_rev| {
        let y: f64 = ny as f64 - y_rev as f64 - 1.0;
        let row: Vec<Vec3> = (0..nx).into_par_iter().map(|x| {
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f64 = (x as f64 + rand::random::<f64>()) / nx as f64;
                let v: f64 = (y as f64 + rand::random::<f64>()) / ny as f64;
                let r: Ray = cam.get_ray(u, v);
                color_vector = color_vector + color(&r, &world, 10);
            }
            color_vector = color_vector/ns as f64;
            color_vector = 255.99*Vec3::new(color_vector.r().sqrt(), color_vector.g().sqrt(), color_vector.b().sqrt());
            color_vector.colorize();
            color_vector
        }).collect();
        bar.inc(1);
        row
    }).collect();

    bar.finish();

    gen_ppm(scene, filename);
}