extern crate rand;

mod vec3;
mod ray;
mod hittable;
mod camera;

use vec3::*;
use ray::*;
use hittable::*;
use camera::Camera;

use std::f32;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

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

fn rand() -> f32 {
    let between = Range::new(-0.5, 0.5);
    return between.ind_sample(&mut rand::thread_rng());
}

fn random_samples(count: usize) -> Vec<(f32, f32, f32)> {
    let mut result: Vec<(f32, f32, f32)> = Vec::with_capacity(count);
    for sample in 0..count {
        result.push((rand(), rand(), 1.0 / count as f32));
    }

    return result;
}

fn fixed_samples() -> Vec<(f32, f32, f32)> {
    vec![
        (0.0, 0.0, 0.4),
        (-0.1, 0.0, 0.15),
        (0.1, 0.0, 0.15),
        (0.0, 0.1, 0.15),
        (0.0, -0.1, 0.15),
    ]
}

fn write_ppm(width: i32, height: i32) -> String {
    const SAMPLE_COUNT: usize = 60;

    let camera = Camera {
        origin: Vec3::new2d(0.0, 0.0),
        lower_left: Vec3::new(-2.0, -1.0, -1.0),
        width: width as f32,
        height: height as f32
    };

    let mut result = String::new();
    result += &format!("P3\n{} {}\n255\n", width, height);
    for x in (0..height + 1).rev() {
        for y in 0..width {
            let mut color_acc = vec3::ZERO;

            for sample in random_samples(SAMPLE_COUNT) {
                let r = camera.ray(y as f32 + sample.0, x as f32 + sample.1);
                color_acc = color_acc + color(r) * sample.2;
            }
            color_acc = color_acc * 255.0;
            result += &format!("{} {} {}\n", color_acc.x as i32, color_acc.y as i32, color_acc.z as i32);
        }
    }
    return result;
}

fn main() {
    println!("{}", write_ppm(400, 200));
}