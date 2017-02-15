use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const V3_ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
pub const V3_ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn new2d(x: f32, y: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
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

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
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


pub fn blend(from: Vec3, to: Vec3, t: f32) -> Vec3 {
    return (1.0 - t) * from + t * to;
}