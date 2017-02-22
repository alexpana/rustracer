extern crate clap;

use clap::{Arg, App, ArgMatches};

pub struct Config {
    pub thread_count: usize,
    pub chunk_count: usize,
    pub scene: String
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
            .arg(Arg::with_name("scene")
                .short("s")
                .long("scene")
                .help("The scene to be rendered")
                .required(true)
                .takes_value(true))
    }

    pub fn new(matches: ArgMatches) -> Config {
        Config {
            thread_count: value_t!(matches.value_of("thread_count"), usize).unwrap_or(8),
            chunk_count: value_t!(matches.value_of("chunk_count"), usize).unwrap_or(100),
            scene: value_t!(matches.value_of("scene"), String).unwrap_or("".to_string())
        }
    }

    pub fn print(&self) {
        println!("Configuration:");
        println!("  thread_count     {}", self.thread_count);
        println!("  chunk_count      {}", self.chunk_count);
        println!("  scene            {}", self.scene);
    }
}