use structures::vec3::Vec3;
#[cfg(test)]
use shading::backgrounds::gen_lerp;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn gen_ppm (image: Vec<Vec<Vec3>>,
                filename: String) -> () {
    let mut content = format!("P3\n{} {}\n255\n",
                              image[0].len(),
                              image.len());
    for row in image {
        for pixel in row {
            content = format!("{}{} {} {}\n",
                              content,
                              pixel.r().to_string(),
                              pixel.g().to_string(),
                              pixel.b().to_string());
        }
    }

    // Time to write to image file!
    let path = Path::new(&filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the content string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}",
                           display,
                           why.description()),
        Ok(_) => println!("successfully wrote to {}",
                          display),
    };
}

#[test]
fn test_gen_ppm() {
    let color_vec: Vec<Vec<Vec3>> = gen_lerp(Vec3{elements:[0.0, 0.0, 0.0]},
                                             Vec3{elements:[-2.0, -1.0, -1.0]},
                                             Vec3{elements:[4.0, 0.0, 0.0]},
                                             Vec3{elements:[0.0, 2.0, 0.0]},
                                             200, 100);
    let filename = "test.ppm".to_string();
    gen_ppm(color_vec, filename);
    assert!(true);

}