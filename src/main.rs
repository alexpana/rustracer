mod vec3;
mod ray;
mod hittable;

use vec3::*;
use ray::*;
use hittable::*;

use std::f32;

const T_MIN: f32 = 0.1;
const T_MAX: f32 = f32::MAX;

fn color(ray: Ray) -> Vec3 {
    let scene: Vec<Box<Hittable>> = vec![
        Box::new(Sphere { origin: Vec3::new(0.0, -100.5, -1.0), radius: 100.0 }),
        Box::new(Sphere { origin: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 }),
        Box::new(Background { color_a: Vec3::new(1.0, 1.0, 1.0), color_b: Vec3::new(0.5, 0.7, 1.0) })
    ];

    let mut closest_hit_distance = T_MAX;
    let mut closest_hit = Option::None;

    for h in scene {
        match (*h).hit(ray, T_MIN, T_MAX) {
            Some(hit) => {
                if hit.t <= closest_hit_distance {
                    closest_hit_distance = hit.t;
                    closest_hit = Some(hit);
                }
            }
            None => ()
        }
    }

    match closest_hit {
        Some(hit) => Vec3::new(
            f32::max(hit.normal.x, 0.0),
            f32::max(hit.normal.y, 0.0),
            f32::max(hit.normal.z, 0.0)),
        None => RED
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