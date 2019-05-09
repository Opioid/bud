use image::Float4;
use std::io::Write;

pub trait Writer {
    fn file_extension(&self) -> &'static str;

    fn write<W: Write>(&self, stream: &mut W, image: &Float4);
}
