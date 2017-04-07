use structures::vec3::Vec3;
use structures::ray::Ray;

pub fn gen_lerp(origin: Vec3, lower_left_corner: Vec3,
        horizontal: Vec3, vertical: Vec3,
        nx: u64, ny: u64) -> Vec<Vec<Vec3>> {
    let mut bg: Vec<Vec<Vec3>> = Vec::new();
    for y in (0..(ny-1)).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let u: f64 = x as f64/nx as f64;
            let v: f64 = y as f64/ny as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col: Vec3 = lerp(&r);
            row.push(255.99*col);
        }
        bg.push(row);
    }
    bg
}

fn lerp(r: &Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5*(unit_direction.y() + 1.0);
    (1.0 - t) * Vec3{elements:[1.0, 1.0, 1.0]} + t * Vec3{elements:[0.5, 0.7, 1.0]}
}

#[test]
fn test_lerp () {
    let vec: Vec<Vec<Vec3>> = gen_lerp(Vec3{elements:[0.0, 0.0, 0.0]},
                                       Vec3{elements:[-2.0, -1.0, -1.0]},
                                       Vec3{elements:[4.0, 0.0, 0.0]},
                                       Vec3{elements:[0.0, 2.0, 0.0]},
                                       10, 5);
    assert_eq!(vec[0][0], Vec3 { elements: [186.29114955690324, 214.17068973414192, 255.99] });
}