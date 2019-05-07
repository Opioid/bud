use image::{self, Float3};
use std::io::Write;

pub struct Writer {}

impl image::Writer for Writer {
    fn file_extension(&self) -> &'static str {
        "png"
    }

    fn write(&self, stream: &mut Write, image: &Float3) {
        println!("This is not a PNG writer");
    }
}
