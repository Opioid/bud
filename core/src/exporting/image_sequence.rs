use super::Sink;
use image::{Float4, Writer};
use std::fs::File;
use std::io::BufWriter;

pub struct ImageSequence<T: Writer> {
    filename: String,
    writer: T,
}

impl<T: Writer> ImageSequence<T> {
    pub fn new(filename: String, writer: T) -> ImageSequence<T> {
        ImageSequence { filename, writer }
    }
}

impl<T: Writer> Sink for ImageSequence<T> {
    fn write(&mut self, image: &Float4) {
        if let Ok(file) = File::create(self.filename.clone() + "." + self.writer.file_extension()) {
            let mut stream = BufWriter::new(file);
            self.writer.write(&mut stream, image);
        }
    }
}
