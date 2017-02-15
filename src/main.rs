use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::io::prelude::*;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

const V3_ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
const V3_ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

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

#[derive(Copy, Clone, Debug)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction.unit() }
    }

    pub fn point_at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

struct HitResult {
    point: Vec3,
    normal: Vec3,
    color: Vec3
}

fn sphere(center: Vec3, radius: f32, ray: Ray) -> Option<HitResult> {
    let oc = ray.origin - center;
    let a = Vec3::dot(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        write_err(t);
        let point_on_sphere = ray.point_at(t);
        return Some(HitResult {
            point: point_on_sphere,
            normal: (point_on_sphere - center).unit(),
            color: V3_ZERO
        })
    }
    return None;
}

fn color(ray: Ray) -> Vec3 {
    let sphere_hit = sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    match sphere_hit {
        Some(hit_result) => {
            return (hit_result.normal + V3_ONE) / 2.0;
        },
        None => {
            let d = ray.direction.unit();

            // transform y from -1..1 to 0..1
            let t = 0.5 * (d.y + 1.0);

            // blend white to blue
            return blend(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t);
        }
    }
}

fn blend(from: Vec3, to: Vec3, t: f32) -> Vec3 {
    return (1.0 - t) * from + t * to;
}


fn write_ppm(width: i32, height: i32) -> String {
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new2d(4.0, 0.0);
    let vertical = Vec3::new2d(0.0, 2.0);
    let origin = Vec3::new2d(0.0, 0.0);

    let mut result = String::new();
    result += &format!("P3\n{} {}\n255\n", width, height);
    for x in (0..height + 1).rev() {
        write_err(height - x);
        for y in 0..width {
            let u = y as f32 / width as f32;
            let v = x as f32 / height as f32;
            let r = Ray::new(origin, lower_left + u * horizontal + v * vertical);
            let col = color(r) * 255.99;
            result += &format!("{} {} {}\n", col.x as i32, col.y as i32, col.z as i32);
        }
    }
    return result;
}

fn write_err<T: Debug>(s: T) {
    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "{:?}", s).expect("Could not write to stderr");
}

fn main() {
//    println!("{}", write_ppm(40, 20));
        println!("{}", write_ppm(400, 200));
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

#[test]
fn ray_point_at() {
    let ray = Ray::new(Vec3::new2d(0.0, 0.0), Vec3::new2d(1.0, 2.0));
    assert_eq!(ray.point_at(2.5), Vec3::new2d(2.5, 5.0))
}

#[test]
fn vec3_dot() {
    assert_eq!(Vec3::dot(Vec3::new(2.0, 3.0, 4.0), Vec3::new(0.5, 0.1, 0.25)), 1.0 + 0.3 + 1.0)
}