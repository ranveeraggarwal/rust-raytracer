use structures::vec3::Vec3;
#[cfg(test)]
use structures::ray::Ray;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

pub fn gen_ppm(image: Vec<Vec<Vec3>>, filename: String) -> () {

    // Time to write to image file!
    let path = Path::new(&filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    write!(file, "P3\n{} {}\n255\n", image[0].len(), image.len()).expect("saving to file failed.");

    let bar = ProgressBar::new((image.len()) as u64);
    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining]    [saving]",
    ));

    for row in image {
        for pixel in row {
            write!(file, "{} {} {}\n", pixel.r(), pixel.g(), pixel.b()).expect("saving to file failed.");
        }
        bar.inc(1);
    }

    bar.finish();
    println!("successfully wrote to {}", display);
}

#[test]
fn test_gen_ppm() {
    fn lerp(r: &Ray) -> Vec3 {
        let unit_direction: Vec3 = r.direction().unit_vector();
        let t: f64 = 0.5*(unit_direction.y() + 1.0);
        (1.0 - t) * Vec3{elements:[1.0, 1.0, 1.0]} + t * Vec3{elements:[0.5, 0.7, 1.0]}
    }

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let nx: u64 = 200;
    let ny: u64 = 100;

    let mut bg: Vec<Vec<Vec3>> = Vec::new();
    for y in (0..ny).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let u: f64 = x as f64/nx as f64;
            let v: f64 = y as f64/ny as f64;
            let r: Ray = Ray::new(origin,
                                  lower_left_corner + u*horizontal + v*vertical);
            let col: Vec3 = lerp(&r);
            let color_vector = Vec3::new((255.99*col.r()).floor(),
                                         (255.99*col.g()).floor(),
                                         (255.99*col.b()).floor());
            row.push(color_vector);
        }
        bg.push(row);
    }

    let filename = "test.ppm".to_string();
    gen_ppm(bg, filename);
    assert!(true);

}
