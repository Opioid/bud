use super::Sink;
use image::Float3;

pub struct Statistics {}

impl Sink for Statistics {
    fn write(&mut self, image: &Float3) {
        println!("Statistics");
    }
}
