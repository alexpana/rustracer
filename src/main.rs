extern crate rand;
extern crate scoped_pool;
#[macro_use(value_t)]
extern crate clap;

mod vec3;
mod ray;
mod hittable;
mod camera;
mod sampling;
mod time;

use vec3::*;
use ray::*;
use hittable::*;
use camera::Camera;
use sampling::*;
use time::Timer;

use clap::{Arg, App, ArgMatches};
use std::f32;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::thread;

const T_MIN: f32 = 0.001;
const T_MAX: f32 = f32::MAX;
const SAMPLE_COUNT: usize = 300;

fn color(ray: Ray, scene: &Scene) -> Vec3 {
    let hit = scene.hit(ray, T_MIN, T_MAX);

    match hit {
        Some(hit) => {
            let reflected = Ray {
                direction: (hit.normal + random_unit_sphere()).unit(),
                origin: hit.point
            };
            return 0.1 * hit.color + 0.5 * color(reflected, scene);
        }
        None => {
            return scene.background.hit(ray, T_MIN, T_MAX).unwrap().color;
        }
    }
}

fn trace(shared_camera: Arc<Camera>, scene: Arc<Scene>, offset: usize, image_buffer: &mut [u8]) {
    let timer = Timer::new();

    //    print!("Started tracing at offset {}\n", offset);

    let camera = &shared_camera;

    for i in 0..(image_buffer.len() / 3) {
        let x = (offset + i) / camera.width;
        let y = (offset + i) % camera.width;
        let mut color_acc = vec3::ZERO;

        for sample in random_samples(SAMPLE_COUNT) {
            let r = camera.ray(y as f32 + sample.0, x as f32 + sample.1);
            color_acc = color_acc + color(r, &scene) * sample.2;
        }

        image_buffer[i * 3 + 0] = (color_acc.x * 255.0) as u8;
        image_buffer[i * 3 + 1] = (color_acc.y * 255.0) as u8;
        image_buffer[i * 3 + 2] = (color_acc.z * 255.0) as u8;
    }

    //    print!("Done tracing at offset {} in {}s\n", offset, timer.count_seconds());
}

fn write_ppm(image_buffer: &[u8], image_size: (usize, usize), file_name: &str) {
    print!("Writing output image to {}\n", file_name);

    let file = File::create(file_name).expect("Could not open output file");
    write!(&file, "P3\n{} {}\n255\n", image_size.0, image_size.1);

    for x in (0..image_size.1).rev() {
        for y in 0..image_size.0 {
            write!(&file, "{} {} {}\n",
                   image_buffer[(x * image_size.0 + y) * 3 + 0],
                   image_buffer[(x * image_size.0 + y) * 3 + 1],
                   image_buffer[(x * image_size.0 + y) * 3 + 2]
            ).unwrap();
        }
    }
}

struct Config {
    thread_count: usize,
    chunk_count: usize
}

impl Config {
    pub fn new(matches: ArgMatches) -> Config {
        Config {
            thread_count: value_t!(matches.value_of("thread_count"), usize).unwrap_or(8),
            chunk_count: value_t!(matches.value_of("chunk_count"), usize).unwrap_or(100)
        }
    }

    pub fn print(&self) {
        println!("Configuration:");
        println!("  thread_count\t{}", self.thread_count);
        println!("  chunk_count\t{}", self.chunk_count);
    }
}

fn main() {
    const CHUNK_COUNT: usize = 100;

    let matches = App::new("rustracer")
        .version("1.0")
        .author("Alexandru Pana <astronothing@gmail.com>")
        .about("Simple Ray Tracer")
        .arg(Arg::with_name("thread_count")
            .short("tc")
            .long("thread_count")
            .help("Sets the number of threads")
            .takes_value(true))
        .arg(Arg::with_name("chunk_count")
            .short("cc")
            .long("chunk_count")
            .help("Sets the number of chunks")
            .takes_value(true))
        .get_matches();

    let config = Config::new(matches);
    config.print();

    let scene = Arc::new(Scene {
        objects: vec![
            Box::new(Sphere { origin: Vec3::new(0.0, -100.5, -2.0), radius: 100.0, color: Vec3::new(0.7, 0.4, 0.5) }),
            Box::new(Sphere { origin: Vec3::new(0.0, 0.0, -2.0), radius: 0.5, color: Vec3::new(0.3, 0.8, 0.4) }),
            Box::new(Sphere { origin: Vec3::new(1.0, 0.0, -2.0), radius: 0.5, color: Vec3::new(1.0, 0.4, 0.4) }),
        ],
        background: Background { color_a: Vec3::new(1.0, 1.0, 1.0), color_b: Vec3::new(0.5, 0.7, 1.0) },
    });

    let camera = Arc::new(Camera {
        origin: Vec3::new2d(0.0, 0.0),
        lower_left: Vec3::new(-1.0, -0.5, -1.0),
        width: 400,
        height: 200
    });

    let mut buffer = vec![0u8; camera.width * camera.height * 3];

    let timer = Timer::new();

    use scoped_pool::Pool;
    let pool = Pool::new(config.thread_count);

    pool.scoped(|scope| {
        let chunk_buffer_size = (camera.width * camera.height * 3) / config.chunk_count;
        let chunks: Vec<&mut [u8]> = buffer.chunks_mut(chunk_buffer_size).collect();

        for (i, mut chunk) in chunks.into_iter().enumerate() {
            let start = i * chunk_buffer_size;
            let shared_camera = camera.clone();
            let shared_scene = scene.clone();

            scope.execute(move || {
                trace(shared_camera, shared_scene, start / 3, &mut chunk);
            });
        }
    });

    println!("All threads finished in {}s", timer.count_seconds());

    write_ppm(&buffer, (camera.width, camera.height), "test2.ppm");
}
