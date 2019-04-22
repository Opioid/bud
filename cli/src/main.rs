extern crate base;
extern crate core;

mod options;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use crate::options::Options;
use base::math::vector2::int2;
use base::math::vector3::float3;
use base::random;
use core::file::System as FileSystem;
use core::image;
use core::image::encoding::rgbe;
use core::take;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);

    let mut file_system = FileSystem::new();

    if options.mounts.is_empty() {
        file_system.push_mount("../data/");
    }

    // let file = File::open(options.take).expect("Unable to find file");
    // let mut stream = BufReader::new(file);

    let stream = file_system.read_stream(&options.take);

    if stream.is_err() {
        println!("got here");
        std::process::exit(1);
    }

    let take = take::Loader::load(&mut stream.unwrap());

    if take.is_err() {}

    let take = take.unwrap();

    println!("{}", take.scene_filename);

    let mut rng = random::Generator::new(0, 0);

    for x in 0..10 {
        println!("Random number {}: {}", x, rng.random_float());
    }

    {
        let file = File::create("image.hdr").expect("Unable to create file");
        let mut stream = BufWriter::new(file);

        let dim = int2::new(512, 256);

        let mut image = image::Float3::new(dim);

        let d = image.dimensions;

        for y in 0..d.y {
            for x in 0..d.x {
                let r = x as f32 / (d.x - 1) as f32;
                let g = y as f32 / (d.y - 1) as f32;

                image.set(x, y, float3::new(r, g, 1.0))
            }
        }

        rgbe::Writer::write(&mut stream, &image);
    }

    let file = File::open("image.hdr").expect("Unable to find file");
    let mut stream = BufReader::new(file);

    let image = rgbe::Reader::read(&mut stream).unwrap();

    println!(
        "Image dimensions {} {}",
        image.dimensions.x, image.dimensions.y
    );

    {
        let file = File::create("copy.hdr").expect("Unable to create file");
        let mut stream = BufWriter::new(file);

        rgbe::Writer::write(&mut stream, &image);
    }
}
