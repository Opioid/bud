use std::io::Write;

use image::Float3;

pub trait Writer {
    fn file_extension(&self) -> &'static str;

    fn write(&self, stream: &mut Write, image: &Float3);
}
