struct Vec3 {
    elements: [f64; 3],
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {elements: [e0, e1, e2]}
    }

    fn x(&self) -> f64 {
        self.elements[0]
    }
    fn y(&self) -> f64 {
        self.elements[1]
    }
    fn z(&self) -> f64 {
        self.elements[2]
    }

    fn r(&self) -> f64 {
        self.elements[0]
    }
    fn g(&self) -> f64 {
        self.elements[1]
    }
    fn b(&self) -> f64 {
        self.elements[2]
    }
}

#[test]
fn test_xyz() {
    let vec: Vec3 = Vec3::new(1.2, 3.4, 6.7);
    assert_eq!(vec.x(), 1.2);
    assert_eq!(vec.y(), 3.4);
    assert_eq!(vec.z(), 6.7);
}

#[test]
fn test_rgb() {
    let vec: Vec3 = Vec3::new(0.2, 0.4, 0.7347845);
    assert_eq!(vec.r(), 0.2);
    assert_eq!(vec.g(), 0.4);
    assert_eq!(vec.b(), 0.7347845);
}