use std::io::Write;

use base::math::vector2::int2;
use base::math::vector3::float3;
use image::{self, Float3};

pub struct Writer {}

impl image::Writer for Writer {
    fn file_extension(&self) -> &'static str {
        "png"
    }

    fn write(&self, stream: &mut Write, image: &Float3) {
        println!("This is not a PNG writer");
    }
}
