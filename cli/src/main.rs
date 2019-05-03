extern crate base;
extern crate core;

mod options;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::time::{Duration, Instant};

use base::chrono;
use base::math::vector2::int2;
use base::math::vector3::float3;
use base::random;
use core::image::encoding::rgbe;
use core::image::{self, Writer};
use core::rendering::driver;

use core::scene::{self, Scene};
use core::take;
use options::Options;

fn main() {
    let total_start = Instant::now();
    
    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);

    let mut scene_loader = scene::Loader::new();

    {
        let file_system = scene_loader.resource_manager().file_system();

        if options.mounts.is_empty() {
            file_system.push_mount("../data/");
        }
    }

    println!("Loading...");

    let loading_start = Instant::now();
    
    let stream = scene_loader
        .resource_manager()
        .file_system()
        .read_stream(&options.take);

    if let Err(err) = stream {
        println!("Loading take \"{}\": {}", options.take, err.message());
        std::process::exit(1);
    }

    let take = take::Loader::load(&mut stream.unwrap());

    if let Err(err) = take {
        println!("Loading take \"{}\": {}", options.take, err.message());
        std::process::exit(1);
    }

    let mut take = take.unwrap();

        let mut scene = Scene::new();

        if let Err(err) = scene_loader.load(&take.scene_filename, &mut scene) {
            println!(
                "Loading take \"{}\": {}",
                take.scene_filename,
                err.message()
            );
            std::process::exit(1);
        }

    println!("Loading time {} s", chrono::duration_to_seconds(loading_start.elapsed()));
    
    println!("Rendering...");

    let rendering_start = Instant::now();
    
        let mut driver = driver::FinalFrame::new(&take);

    driver.render(&scene, &mut take.view, &mut take.exporters);

    println!("Total render time {} s", chrono::duration_to_seconds(rendering_start.elapsed()));

    println!("Total elapsed time {} s", chrono::duration_to_seconds(total_start.elapsed()));
}
