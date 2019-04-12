use std::io::Write;

pub struct Writer {}

impl Writer {
    pub fn write(stream: &mut Write, width: i32, height: i32) {
        Writer::write_header(stream, width, height);

        Writer::write_pixels(stream, width, height);
    }

    fn write_header(stream: &mut Write, width: i32, height: i32) {
        stream.write(b"#?RGBE\n").unwrap();
        stream.write(b"FORMAT=32-bit_rle_rgbe\n\n").unwrap();
        write!(stream, "-Y {} +X {}\n", height, width).unwrap();
    }

    fn write_pixels(stream: &mut Write, width: i32, height: i32) {
        for _ in 0..width * height {
            stream.write(&Writer::float_to_rgbe()).unwrap();
        }
    }

    fn float_to_rgbe() -> [u8; 4] {
        [0, 0, 0, 0]
    }
}
