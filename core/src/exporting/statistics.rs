use super::Sink;
use image::Float4;

pub struct Statistics {}

impl Sink for Statistics {
    fn write(&mut self, _image: &Float4) {
        println!("Statistics");
    }
}
