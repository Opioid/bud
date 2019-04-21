use std::io::Write;

use crate::base::math::vector2::int2;
use crate::base::math::vector3::float3;
use crate::core::image::Float3;

pub struct Writer {}

// http://www.graphics.cornell.edu/~bjw/rgbe

impl Writer {
    pub fn write(stream: &mut Write, image: &Float3) {
        Writer::write_header(stream, image.dimensions);

        Writer::write_pixels_rle(stream, image);
    }

    fn write_header(stream: &mut Write, dimensions: int2) {
        stream.write(b"#?RGBE\n").unwrap();
        stream.write(b"FORMAT=32-bit_rle_rgbe\n\n").unwrap();
        write!(stream, "-Y {} +X {}\n", dimensions.y, dimensions.x).unwrap();
    }

    fn write_pixels(stream: &mut Write, image: &Float3) {
        let d = image.dimensions;
        for i in 0..d.x * d.y {
            let rgbe = Writer::float_to_rgbe(image.get_by_index(i));
            stream.write(&rgbe).unwrap();
        }
    }

    fn write_pixels_rle(stream: &mut Write, image: &Float3) {
        let scanline_width = image.dimensions.x;
        let num_scanlines = image.dimensions.y;

        if scanline_width < 8 || scanline_width > 0x7FFF {
            return Writer::write_pixels(stream, image);
        }

        let mut buffer = vec![0u8; (scanline_width * 4) as usize];

        let mut current_pixel = 0i32;

        for _ in 0..num_scanlines {
            let mut rgbe = [
                2u8,
                2u8,
                (scanline_width >> 8) as u8,
                (scanline_width & 0xFF) as u8,
            ];

            stream.write(&rgbe).unwrap();

            for i in 0..scanline_width {
                let pixel = image.get_by_index(current_pixel);

                rgbe = Writer::float_to_rgbe(pixel);

                buffer[(i + scanline_width * 0) as usize] = rgbe[0];
                buffer[(i + scanline_width * 1) as usize] = rgbe[1];
                buffer[(i + scanline_width * 2) as usize] = rgbe[2];
                buffer[(i + scanline_width * 3) as usize] = rgbe[3];

                current_pixel += 1;
            }

            // write out each of the four channels separately run length encoded
            // first red, then green, then blue, then exponent
            for i in 0..4 {
                let start = (i * scanline_width) as usize;
                Writer::write_bytes_rle(stream, &buffer[start..start + scanline_width as usize]);
            }
        }
    }

    // The code below is only needed for the run-length encoded files.
    // Run length encoding adds considerable complexity but does save some space.
    // For each scanline, each channel (r,g,b,e) is encoded separately for better compression.
    fn write_bytes_rle(stream: &mut Write, data: &[u8]) {
        let min_run_length = 4;

        let mut buffer = [0u8, 0u8];

        let mut current = 0;

        while current < data.len() {
            let mut begin_run = current;

            // find next run of length at least 4 if one exists
            let mut run_count = 0;
            let mut old_run_count = 0;

            while run_count < min_run_length && begin_run < data.len() {
                begin_run += run_count;
                old_run_count = run_count;
                run_count = 1;

                while begin_run + run_count < data.len()
                    && run_count < 127
                    && data[begin_run] == data[begin_run + run_count]
                {
                    run_count += 1;
                }
            }

            // if data before next big run is a short run then write it as such
            if old_run_count > 1 && old_run_count == begin_run - current {
                // write short run
                buffer[0] = (128 + old_run_count) as u8;
                buffer[1] = data[current];

                stream.write(&buffer).unwrap();

                current = begin_run;
            }

            // write out bytes until we reach the start of the next run
            while current < begin_run {
                let mut nonrun_count = begin_run - current;

                if nonrun_count > 128 {
                    nonrun_count = 128;
                }

                buffer[0] = nonrun_count as u8;

                stream.write(&buffer[0..1]).unwrap();

                stream
                    .write(&data[current..current + nonrun_count])
                    .unwrap();

                current += nonrun_count;
            }

            // write out next run if one was found
            if run_count >= min_run_length {
                buffer[0] = (128 + run_count) as u8;
                buffer[1] = data[begin_run];

                stream.write(&buffer).unwrap();

                current += run_count;
            }
        }
    }

    // Not a complete implementation of frexp - only for positive numbers
    fn frexp(s: f32) -> (f32, i32) {
        if 0.0 == s {
            return (s, 0);
        } else {
            let lg = s.log2();
            let x = (lg - lg.floor() - 1.0).exp2();
            let exp = lg.floor() as i32 + 1;
            (x, exp)
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
