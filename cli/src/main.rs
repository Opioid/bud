extern crate base;
extern crate core;

mod options;

use std::env;
use std::fs::File;
use std::io::BufWriter;

use base::math::vector2::int2;
use base::math::vector3::float3;
use base::random;
use core::error::Error;
use core::image::encoding::rgbe;
use core::image::{self, Writer};
use core::rendering::driver;

use core::scene::{self, Scene};
use core::take;
use options::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);

    let mut scene_loader = scene::Loader::new();

    {
        let file_system = scene_loader.resource_manager().file_system();

        if options.mounts.is_empty() {
            file_system.push_mount("../data/");
        }
    }

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

    {
        let mut scene = Scene::new();

        if let Err(err) = scene_loader.load(&take.scene_filename, &mut scene) {
            println!(
                "Loading take \"{}\": {}",
                take.scene_filename,
                err.message()
            );
            std::process::exit(1);
        }

        let mut driver = driver::FinalFrame::new(take.view.camera.sensor_dimensions(), &scene);

        driver.render(&mut take.view, &mut take.exporters);
    }

    let mut rng = random::Generator::new(0, 0);

    for x in 0..10 {
        println!("Random number {}: {}", x, rng.random_float());
    }

    let writer = rgbe::Writer {};

    {
        let file = File::create("image.hdr").expect("Unable to create file");
        let mut stream = BufWriter::new(file);

        let dim = int2::new(512, 256);

        let mut image = image::Float3::new(dim);

        let d = image.dimensions;

        for y in 0..d.v[1] {
            for x in 0..d.v[0] {
                let r = x as f32 / (d.v[0] - 1) as f32;
                let g = y as f32 / (d.v[1] - 1) as f32;

                image.set(x, y, float3::new(r, g, 1.0))
            }
        }

        writer.write(&mut stream, &image);
    }

    let mut stream = scene_loader
        .resource_manager()
        .file_system()
        .read_stream("image.hdr")
        .expect("Unable to find file");

    let image = rgbe::Reader::read(&mut stream).unwrap();

    println!(
        "Image dimensions {} {}",
        image.dimensions.v[0], image.dimensions.v[1]
    );

    {
        let file = File::create("copy.hdr").expect("Unable to create file");
        let mut stream = BufWriter::new(file);

        writer.write(&mut stream, &image);
    }
}
