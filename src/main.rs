mod vec3;
mod ray;
mod hittable;

use vec3::*;
use ray::*;
use hittable::*;

fn color(ray: Ray) -> Vec3 {
    let s = Sphere { origin: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 };
    let b = Background { color_a: Vec3::new(1.0, 1.0, 1.0), color_b: Vec3::new(0.5, 0.7, 1.0) };

    let sphere_hit = s.hit(ray);
    match sphere_hit {
        Some(hit_result) => {
            return (hit_result.normal + V3_ONE) / 2.0;
        },
        None => {
            return b.hit(ray).unwrap().color;
        }
    }
}

fn write_ppm(width: i32, height: i32) -> String {
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new2d(4.0, 0.0);
    let vertical = Vec3::new2d(0.0, 2.0);
    let origin = Vec3::new2d(0.0, 0.0);

    let mut result = String::new();
    result += &format!("P3\n{} {}\n255\n", width, height);
    for x in (0..height + 1).rev() {
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

fn main() {
    println!("{}", write_ppm(400, 200));
}

// Tests

#[test]
fn vector_scalar_division() {
    assert_eq! (Vec3::new(10.0, 20.0, 30.0) / 10.0, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn vector_length() {
    assert_eq! (Vec3::new(10.0, 10.0, 10.0).length(), 17.320509);
}

#[test]
fn vector_unit() {
    assert_eq! (Vec3::new(10.0, 20.0, 30.0).unit(), Vec3::new(0.2672, 0.5345, 0.8017));
}

#[test]
fn ray_point_at() {
    let ray = Ray::new(Vec3::new2d(0.0, 0.0), Vec3::new2d(1.0, 2.0));
    assert_eq! (ray.point_at(2.5), Vec3::new2d(1.0, 2.0).unit() * 2.5)
}

#[test]
fn vec3_dot() {
    assert_eq! (Vec3::dot(Vec3::new(2.0, 3.0, 4.0), Vec3::new(0.5, 0.1, 0.25)), 1.0 + 0.3 + 1.0)
}