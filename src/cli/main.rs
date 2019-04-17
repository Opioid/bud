extern crate bud;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use bud::base::math::vector2::int2;
use bud::base::math::vector3::float3;
use bud::base::random;
use bud::core::image;
use bud::core::image::encoding::rgbe;

fn main() {
    let mut rng = random::Generator::new(0, 0);

    for x in 0..10 {
        println!("Random number {}: {}", x, rng.random_float());
    }

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

    let file = File::open("image.hdr").expect("Unable to find file");
    let mut stream = BufReader::new(file);

    let image = rgbe::Reader::read(&mut stream).unwrap();

    println!(
        "Image dimensions {} {}",
        image.dimensions.x, image.dimensions.y
    );
}
