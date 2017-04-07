extern crate raytracer;

use raytracer::structures::vec3::Vec3;
use raytracer::structures::ray::Ray;

use raytracer::objects::sphere::Sphere;

use raytracer::io::write::gen_ppm;

fn main() {
    let filename = "outputs/basic_sphere.ppm".to_string();
    
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let nx: u64 = 200;
    let ny: u64 = 100;

    let sphere_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere: Sphere = Sphere::new(sphere_center, 0.5);

    let mut scene: Vec<Vec<Vec3>> = Vec::new();

    for y in (0..ny).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let u: f64 = x as f64/nx as f64;
            let v: f64 = y as f64/ny as f64;
            let r: Ray = Ray::new(origin,
                                  lower_left_corner + u*horizontal + v*vertical);
            let mut color_vector: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            if sphere.intersects(&r) {
                color_vector = Vec3::new(0.0, 0.0, 255.0);
            }
            row.push(color_vector);
        }
        scene.push(row);
    }
    gen_ppm(scene, filename);
}