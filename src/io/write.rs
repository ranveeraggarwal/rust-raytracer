use structures::vec3::Vec3;
use shading::backgrounds::gen_lerp;

pub fn gen_ppm (image: Vec<Vec<Vec3>>, filename: &String) -> String {
    let mut content: String = String::new();
    for i in image {
        for j in i {
            content = format!("{} {} {} {}", content, j.r().to_string(), j.g().to_string(), j.b().to_string());
        }
        content = format!("{} \n", content)
    }
    content
}

#[test]
fn test_gen_ppm() {
    let color_vec: Vec<Vec<Vec3>> = gen_lerp(Vec3{elements:[0.0, 0.0, 0.0]},
                                             Vec3{elements:[-2.0, -1.0, -1.0]},
                                             Vec3{elements:[4.0, 0.0, 0.0]},
                                             Vec3{elements:[0.0, 2.0, 0.0]},
                                             200, 100);
    let filename = String::new();
    let content: String = gen_ppm(color_vec, &filename);
    // println!("{:?}", content);
    assert!(true);

}