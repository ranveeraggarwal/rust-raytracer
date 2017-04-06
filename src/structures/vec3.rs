use std::ops::{Add, Sub};
use std::cmp::PartialEq;

#[derive(Debug)]
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

    fn print(&self) -> () {
        println!("{:?}", self);
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {elements: [self.x()+other.x(), self.y()+other.y(), self.z()+other.z()]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {elements: [self.x()-other.x(), self.y()-other.y(), self.z()-other.z()]}
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Vec3 {
    fn dot(&self, other: &Vec3) -> f64 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
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
fn test_dot() {
    let vec_1: Vec3 = Vec3::new(0.2, 0.4, 0.7);
    let vec_2: Vec3 = Vec3::new(0.1, 0.3, 0.3);
    assert_eq!(vec_1.dot(&vec_2), 0.35);
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
