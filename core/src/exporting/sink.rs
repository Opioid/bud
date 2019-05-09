use image::Float4;

pub trait Sink {
    fn write(&mut self, image: &Float4);
}
