extern crate raytracer;

use raytracer::structures::vec3::Vec3;
use raytracer::shading::backgrounds::gen_lerp;
use raytracer::io::write::gen_ppm;

fn main() {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let color_vec: Vec<Vec<Vec3>> = gen_lerp(origin,
                                             lower_left_corner,
                                             horizontal,
                                             vertical,
                                             200, 100);
    let filename = "test.ppm".to_string();
    gen_ppm(color_vec, filename);
}