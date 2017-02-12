use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }
}

impl PartialEq<Vec3> for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        const THRESHOLD: f32 = 0.0001; // consts are inlined
        return (self.x - other.x).abs() < THRESHOLD &&
            (self.y - other.y).abs() < THRESHOLD &&
            (self.y - other.y).abs() < THRESHOLD;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

fn write_ppm(width: i32, height: i32) -> String {
    let mut result = String::new();
    result += &format!("P3\n{} {}\n255\n", width, height);
    for x in (0..height + 1).rev() {
        for y in 0..width {
            let col = Vec3::new(y as f32 / width as f32, x as f32 / height as f32, 0.2) * 255.99;
            result += &format!("{} {} {}\n", col.x as i32, col.y as i32, col.z as i32);
        }
    }
    return result;
}

fn main() {
    println!("{}", write_ppm(200, 100));
}


// Tests

#[test]
fn vector_scalar_division() {
    assert_eq!(Vec3::new(10.0, 20.0, 30.0) / 10.0, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn vector_length() {
    assert_eq!(Vec3::new(10.0, 10.0, 10.0).length(), 17.320509);
}

#[test]
fn vector_unit() {
    assert_eq!(Vec3::new(10.0, 20.0, 30.0).unit(), Vec3::new(0.2672, 0.5345, 0.8017));
}
