extern crate clap;

use clap::{Arg, App, ArgMatches};

#[derive(Clone)]
pub struct Config {
    pub thread_count: usize,
    pub chunk_count: usize,
    pub sample_count: usize,
    pub scene: String,
    pub output_image: String,
    pub output_size: (usize, usize),
}

impl Config {
    pub fn get_app<'a>() -> App<'a, 'a> {
        App::new("rustracer")
            .version("1.0")
            .author("Alexandru Pana <astronothing@gmail.com>")
            .about("Simple Ray Tracer")
            .arg(Arg::with_name("thread_count")
                .short("t")
                .long("thread_count")
                .help("Sets the number of threads")
                .takes_value(true))
            .arg(Arg::with_name("chunk_count")
                .short("c")
                .long("chunk_count")
                .help("Sets the number of chunks")
                .takes_value(true))
            .arg(Arg::with_name("sample_count")
                .short("s")
                .long("sample_count")
                .help("Sets the number of samples per pixel")
                .takes_value(true))
            .arg(Arg::with_name("scene")
                .long("scene")
                .help("The scene to be rendered")
                .takes_value(true))
            .arg(Arg::with_name("output_width")
                .long("output_width")
                .help("The output image width")
                .takes_value(true))
            .arg(Arg::with_name("output_height")
                .long("output_height")
                .help("The output image height")
                .takes_value(true))
            .arg(Arg::with_name("output_image")
                .long("output_image")
                .help("The output image")
                .takes_value(true))
    }

    pub fn new(matches: ArgMatches) -> Config {
        Config {
            thread_count: value_t!(matches.value_of("thread_count"), usize).unwrap_or(8),
            chunk_count: value_t!(matches.value_of("chunk_count"), usize).unwrap_or(100),
            sample_count: value_t!(matches.value_of("sample_count"), usize).unwrap_or(300),
            scene: value_t!(matches.value_of("scene"), String).unwrap_or("".to_string()),
            output_image: value_t!(matches.value_of("output_image"), String).unwrap_or("output.ppm".to_string()),
            output_size: (value_t!(matches.value_of("output_width"), usize).unwrap_or(400),
                          value_t!(matches.value_of("output_height"), usize).unwrap_or(200)),
        }
    }

    pub fn print(&self) {
        println!();
        println!("Configuration:");
        println!("  thread_count     {}", self.thread_count);
        println!("  chunk_count      {}", self.chunk_count);
        println!("  sample_count     {}", self.sample_count);
        println!("  scene            {}", self.scene);
        println!("  output_image     {}", self.output_image);
        println!("  output_size      {:?}", self.output_size);
        println!();
    }
}