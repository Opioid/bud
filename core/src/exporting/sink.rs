use image::Float3;

pub trait Sink {
    fn write(&mut self, image: &Float3);
}
