use std::io::Write;

use crate::base::math::vector2::int2;
use crate::base::math::vector3::float3;
use crate::core::image;

pub struct Writer {}

impl Writer {
    pub fn write(stream: &mut Write, image: &image::Float3) {
        Writer::write_header(stream, image.dimensions);

        Writer::write_pixels(stream, image);
    }

    fn write_header(stream: &mut Write, dimensions: int2) {
        stream.write(b"#?RGBE\n").unwrap();
        stream.write(b"FORMAT=32-bit_rle_rgbe\n\n").unwrap();
        write!(stream, "-Y {} +X {}\n", dimensions.y, dimensions.x).unwrap();
    }

    fn write_pixels(stream: &mut Write, image: &image::Float3) {
        let d = image.dimensions;
        for i in 0..d.x * d.y {
            stream
                .write(&Writer::float_to_rgbe(image.get_by_index(i)))
                .unwrap();
        }
    }

    fn frexp(s: f32) -> (f32, i32) {
        if 0.0 == s {
            return (s, 0);
        } else {
            let lg = s.abs().log2();
            let x = (lg - lg.floor() - 1.0).exp2();
            let exp = lg.floor() + 1.0;
            (s.signum() * x, exp as i32)
        }
    }

    fn float_to_rgbe(c: float3) -> [u8; 4] {
        let mut v = c.x;

        if c.y > v {
            v = c.y
        }

        if c.z > v {
            v = c.z
        }

        if v < 1e-32 {
            return [0, 0, 0, 0];
        } else {
            let (f, e) = Writer::frexp(v);

            v = f * 256.0 / v;

            return [
                (c.x * v) as u8,
                (c.y * v) as u8,
                (c.z * v) as u8,
                (e + 128) as u8,
            ];
        }
    }
}
