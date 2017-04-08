use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elements: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {elements: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }
    pub fn y(&self) -> f64 {
        self.elements[1]
    }
    pub fn z(&self) -> f64 {
        self.elements[2]
    }

    pub fn r(&self) -> f64 {
        self.elements[0]
    }
    pub fn g(&self) -> f64 {
        self.elements[1]
    }
    pub fn b(&self) -> f64 {
        self.elements[2]
    }

    pub fn print(&self) -> () {
        println!("{:?}", self);
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {elements: [self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {elements: [self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()]}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {elements: [self.x() * other,
            self.y() * other,
            self.z() * other]}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {elements: [self * other.x(),
            self * other.y(),
            self * other.z()]}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        if other == 0.0 {
            return Vec3 {elements: [f64::MAX, f64::MAX, f64::MAX]};
        }
        Vec3 {elements: [self.x() / other,
            self.y() / other,
            self.z() / other]}
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3{elements: [self.x() / self.length(),
            self.y() / self.length(),
            self.z() / self.length()]}
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {elements: [self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()]}
    }

    /// Converts floats to ints, colors are
    /// supposed to be integer values between 0 and 255.
    pub fn colorize(&mut self) -> () {
        self.elements[0] = self.elements[0].floor();
        self.elements[1] = self.elements[1].floor();
        self.elements[2] = self.elements[2].floor();
    }
}

#[test]
fn test_gen() {
    let vec: Vec3 = Vec3::new(0.2, 0.4, 0.8);
    vec.print();
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

#[test]
fn test_add() {
    let vec_1: Vec3 = Vec3::new(0.2, 0.4, 0.7);
    let vec_2: Vec3 = Vec3::new(0.1, 0.3, 0.3);
    let vec_test: Vec3 = Vec3::new(0.30000000000000004, 0.7, 1.0);
    assert_eq!(vec_1 + vec_2, vec_test);
}

#[test]
fn test_sub() {
    let vec_1: Vec3 = Vec3::new(0.2, 0.4, 0.8);
    let vec_2: Vec3 = Vec3::new(0.1, 0.2, 0.4);
    let vec_test: Vec3 = Vec3::new(0.1, 0.2, 0.4);
    assert_eq!(vec_1-vec_2, vec_test);
}

#[test]
fn test_mul() {
    let vec: Vec3 = Vec3::new(0.2, 0.4, 0.8);
    let vec_test: Vec3 = Vec3::new(0.4, 0.8, 1.6);
    assert_eq!(vec*2.0, vec_test);
    assert_eq!(2.0*vec, vec_test);
}

#[test]
fn test_div() {
    let vec: Vec3 = Vec3::new(0.2, 0.4, 0.8);
    let vec_test: Vec3 = Vec3::new(0.1, 0.2, 0.4);
    let vec_inf: Vec3 = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
    assert_eq!(vec/2.0, vec_test);
    assert_eq!(vec/0.0, vec_inf);
}

#[test]
fn test_len() {
    let vec: Vec3 = Vec3::new(0.0, 3.0, 4.0);
    assert_eq!(vec.length(), 5.0);
}

#[test]
fn test_unit() {
    let vec: Vec3 = Vec3::new(0.0, 3.0, 4.0);
    let unit_vec: Vec3 = Vec3::new(0.0, 3.0/5.0, 4.0/5.0);
    assert_eq!(vec.unit_vector(), unit_vec);
}

#[test]
fn test_dot() {
    let vec_1: Vec3 = Vec3::new(0.2, 0.4, 0.7);
    let vec_2: Vec3 = Vec3::new(0.1, 0.3, 0.3);
    assert_eq!(vec_1.dot(&vec_2), 0.35);
}

#[test]
fn test_cross() {
    let vec_1: Vec3 = Vec3::new(3.0, -3.0, 1.0);
    let vec_2: Vec3 = Vec3::new(4.0, 9.0, 2.0);
    let vec_cross: Vec3 = Vec3::new(-15.0, -2.0, 39.0);
    assert_eq!(vec_1.cross(&vec_2), vec_cross);
}

#[test]
fn test_colorize() {
    let mut vec: Vec3 = Vec3::new(3.23, 3.1, 1.0);
    vec.colorize();
    assert_eq!(vec, Vec3::new(3.0, 3.0, 1.0));
}
