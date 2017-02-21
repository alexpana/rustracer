extern crate rand;

use rand::distributions::{IndependentSample, Range};

use vec3::Vec3;

pub fn random_samples(count: usize) -> Vec<(f32, f32, f32)> {
    let mut result: Vec<(f32, f32, f32)> = Vec::with_capacity(count);
    for sample in 0..count {
        result.push((rand() - 0.5, rand() - 0.5, 1.0 / count as f32));
    }

    return result;
}

pub fn fixed_samples() -> Vec<(f32, f32, f32)> {
    vec![
        (0.0, 0.0, 0.4),
        (-0.1, 0.0, 0.15),
        (0.1, 0.0, 0.15),
        (0.0, 0.1, 0.15),
        (0.0, -0.1, 0.15),
    ]
}

pub fn rand() -> f32 {
    let between = Range::new(0.0, 1.0);
    return between.ind_sample(&mut rand::thread_rng());
}

pub fn random_unit_sphere() -> Vec3 {
    return Vec3 {
        x: rand() - 0.5,
        y: rand() - 0.5,
        z: rand() - 0.5
    }.unit();
}