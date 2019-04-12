extern crate bud;

use std::fs::File;
use std::io::BufWriter;

use bud::base::random;
use bud::core::image::encoding::rgbe;

fn main() {
    let mut rng = random::Generator::new(0, 0);

    for x in 0..10 {
        println!("Random number {}: {}", x, rng.random_float());
    }

    let file = File::create("image.hdr").expect("Unable to create file");
    let mut stream = BufWriter::new(file);

    let width = 512;
    let height = 256;

    rgbe::Writer::write(&mut stream, width, height);
}
