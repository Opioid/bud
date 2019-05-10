use base::encoding;
use base::math::{byte3, byte4, float3};
use base::spectrum::srgb;
use image::{self, Float4};
use miniz_oxide::deflate;
use std::io::Write;
use std::slice;

pub struct Writer {}

impl image::Writer for Writer {
    fn file_extension(&self) -> &'static str {
        "png"
    }

    fn write<W: Write>(&self, stream: &mut W, image: &Float4) {
        let d = image.dimensions;

        let num_pixels = d.v[0] * d.v[1];

        let mut srgb = vec![byte3::identity(); num_pixels as usize];

        for i in 0..num_pixels {
            let c = image.get_by_index(i).xyz();

            let c = srgb::linear_to_gamma_3(c);

            srgb[i as usize] = byte3::from(encoding::float_to_unorm_3(c));
        }

        let byte_slice =
            unsafe { slice::from_raw_parts(srgb.as_ptr() as *const u8, (num_pixels * 3) as usize) };

        write_u8(stream, byte_slice, d.v[0] as u32, d.v[1] as u32, ColorType::Truecolor);
    }
}

pub struct WriterAlpha {}

impl image::Writer for WriterAlpha {
    fn file_extension(&self) -> &'static str {
        "png"
    }

    fn write<W: Write>(&self, stream: &mut W, image: &Float4) {
        let d = image.dimensions;

        let num_pixels = d.v[0] * d.v[1];

        let mut srgb = vec![byte4::identity(); num_pixels as usize];

        for i in 0..num_pixels {
            let c = image.get_by_index(i);

            let c = srgb::linear_to_gamma_4(c);

            srgb[i as usize] = encoding::float_to_unorm_4(c);
        }

        let byte_slice =
            unsafe { slice::from_raw_parts(srgb.as_ptr() as *const u8, (num_pixels * 4) as usize) };

        write_u8(stream, byte_slice, d.v[0] as u32, d.v[1] as u32, ColorType::TruecolorAlpha);
    }
}

mod crc32 {
    // https://github.com/ledbettj/crc32/blob/master/rust/src/crc32.rs
    pub struct Crc32 {
        table: [u32; 256],
        value: u32,
    }

    const CRC32_INITIAL: u32 = 0xedb88320;

    impl Crc32 {
        pub fn new() -> Crc32 {
            let mut c = Crc32 { table: [0; 256], value: 0xffffffff };
            for i in 0..256 {
                let mut v = i as u32;
                for _ in 0..8 {
                    v = if v & 1 != 0 { CRC32_INITIAL ^ (v >> 1) } else { v >> 1 }
                }
                c.table[i] = v;
            }
            return c;
        }

        pub fn start(&mut self) {
            self.value = 0xffffffff;
        }

        pub fn update(&mut self, buf: &[u8]) {
            for &i in buf {
                self.value =
                    self.table[((self.value ^ (i as u32)) & 0xff) as usize] ^ (self.value >> 8);
            }
        }

        pub fn finalize(&mut self) -> u32 {
            self.value ^ 0xffffffff_u32
        }

        #[allow(dead_code)]
        pub fn crc(&mut self, buf: &[u8]) -> u32 {
            self.start();
            self.update(buf);
            self.finalize()
        }
    }
}

// big endian
#[inline]
fn u32_to_u8_be(v: u32) -> [u8; 4] {
    [(v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8]
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ColorType {
    _Grayscale = 0,
    Truecolor = 2,
    _Palleted = 3,
    _GrayscaleAlpha = 4,
    TruecolorAlpha = 6,
}

// Based on
// https://github.com/ideasman42/png-encode-mini-rs
// Write RGBA pixels to uncompressed PNG.
pub fn write_u8<W: Write>(stream: &mut W, image: &[u8], w: u32, h: u32, ct: ColorType) {
    fn write_chunk<W: Write>(stream: &mut W, png_tag: &[u8; 4], data: &[u8]) {
        stream.write(&u32_to_u8_be(data.len() as u32)).unwrap();
        stream.write(png_tag).unwrap();
        stream.write(data).unwrap();
        {
            let mut crc = crc32::Crc32::new();
            crc.start();
            crc.update(png_tag);
            crc.update(data);
            stream.write(&u32_to_u8_be(crc.finalize())).unwrap();
        }
    }

    stream.write(b"\x89PNG\r\n\x1a\n").unwrap();
    {
        let wb = u32_to_u8_be(w);
        let hb = u32_to_u8_be(h);
        let data = [
            wb[0], wb[1], wb[2], wb[3], // width
            hb[0], hb[1], hb[2], hb[3],    // height
            8,        // color depth
            ct as u8, // color type
            0, 0, 0,
        ];
        write_chunk(stream, b"IHDR", &data);
    }

    {
        let bytes_per_pixel = match ct {
            ColorType::Truecolor => 3,
            _ => 4,
        };

        let row_bytes = w * bytes_per_pixel;
        let final_len = (row_bytes + 1) * h;
        let mut raw_data = Vec::with_capacity(final_len as usize);
        let mut span = 0;
        for r in 0..h {
            raw_data.push(0);
            raw_data.extend_from_slice(&image[span as usize..(span + row_bytes) as usize]);

            span += row_bytes;
        }

        write_chunk(stream, b"IDAT", &deflate::compress_to_vec_zlib(&raw_data, 6));
    }

    write_chunk(stream, b"IEND", &[]);
}
