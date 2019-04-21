use std::io;
use std::io::{BufRead, Error, ErrorKind};

use crate::base::math::vector2::int2;
use crate::base::math::vector3::float3;
use crate::core::image::Float3;

pub struct Reader {}

struct Header {
    width: u32,
    height: u32,
}

impl Reader {
    pub fn read(stream: &mut BufRead) -> io::Result<Float3> {
        let header = Reader::read_header(stream)?;

        let mut image = Float3::new(int2::new(header.width as i32, header.height as i32));

        Reader::read_pixels_rle(stream, header.width, header.height, &mut image)?;

        Ok(image)
    }

    fn read_header(stream: &mut BufRead) -> io::Result<Header> {
        let mut line = String::new();

        stream.read_line(&mut line)?;

        if line.len() < 2 || &line.as_bytes()[..2] != "#?".as_bytes() {
            return Err(Error::new(ErrorKind::Other, "Bad initial token"));
        }

        let mut format_specifier = false;

        loop {
            line.clear();
            stream.read_line(&mut line)?;

            if line.is_empty() || line.as_bytes()[0] == '\n' as u8 {
                break;
            } else if "FORMAT=32-bit_rle_rgbe\n" == line {
                format_specifier = true;
            }
        }

        if !format_specifier {
            return Err(Error::new(ErrorKind::Other, "No format specifier found"));
        }

        line.clear();
        stream.read_line(&mut line)?;

        if let Some((width, height)) = Reader::parse_size(&line) {
            return Ok(Header { width, height });
        }

        Err(Error::new(ErrorKind::Other, "Missing image size specifier"))
    }

    fn parse_size(line: &str) -> Option<(u32, u32)> {
        let mut tokens = line.split(' ');

        if "-Y" != tokens.next()? {
            return None;
        }

        let height = tokens.next()?.parse::<u32>();
        if height.is_err() {
            return None;
        }

        if "+X" != tokens.next()? {
            return None;
        }

        let width = tokens.next()?.trim().parse::<u32>();
        if width.is_err() {
            return None;
        }

        Some((width.unwrap(), height.unwrap()))
    }

    fn read_pixels_rle(stream: &mut BufRead, scanline_width : u32, num_scanlines : u32, image: &mut Float3) -> io::Result<()> {
        if scanline_width < 8 || scanline_width > 0x7FFF {
            return Reader::read_pixels(stream, scanline_width * num_scanlines, image, 0);
        }

        let mut offset = 0;
        let mut rgbe = [0u8, 0u8, 0u8, 0u8];
        let mut buf = [0u8, 0u8];

        let mut scanline_buffer = vec!(0u8; 4 * scanline_width as usize);

        for _ in 0..num_scanlines {
            stream.read_exact(&mut rgbe)?;

            if rgbe[0] != 2 || rgbe[1] != 2 || (rgbe[2] & 0x80) != 0 {
                // This file is not run length encoded
                let color = Reader::rgbe_to_float(rgbe);

                image.set_by_index(0, color);

                return Reader::read_pixels(stream, scanline_width * num_scanlines - 1, image, 1);
            }

            if (rgbe[2] as u32) << 8 | (rgbe[3] as u32) != scanline_width {
                return Err(Error::new(ErrorKind::Other, "Wrong scanline width"));
            }

            // read each of the four channels for the scanline into the buffer
            let mut index = 0usize;
            for i in 0..4 {
                let end = (i + 1) * scanline_width as usize;

                while index < end {
                    stream.read_exact(&mut buf)?;

                    if buf[0] > 128 {
                        // a run of the same value
                        let count = buf[0] as usize - 128;

                        if count == 0 || count > end - index {
                            return Err(Error::new(ErrorKind::Other, "Bad scanline data"));
                        }

                        for _ in 0..count {
                            scanline_buffer[index] = buf[1];
                            index += 1;
                        }
                    } else {
                        // a non-run
                        let mut count = buf[0] as usize;

                        if count == 0 || count > end - index {
                            return Err(Error::new(ErrorKind::Other, "Bad scanline data"));
                        }

                        scanline_buffer[index] = buf[1];
                        index += 1;

                        count -= 1;
                        if count > 0 {
                            stream.read_exact(&mut scanline_buffer[index..index + count])?;

                            index += count;
                        }
                    }
                }
            }

            // now convert data from buffer into floats
            for i in 0..scanline_width {
                rgbe[0] = scanline_buffer[(i + 0 * scanline_width) as usize];
                rgbe[1] = scanline_buffer[(i + 1 * scanline_width) as usize];
                rgbe[2] = scanline_buffer[(i + 2 * scanline_width) as usize];
                rgbe[3] = scanline_buffer[(i + 3 * scanline_width) as usize];

                image.set_by_index(offset, Reader::rgbe_to_float(rgbe));
                offset += 1;
            }
        }

        Ok(())
    }

    fn read_pixels(stream: &mut BufRead, num_pixels: u32, image: &mut Float3, offset: u32) -> io::Result<()> {
        let mut rgbe = [0u8, 0u8, 0u8, 0u8];

        let mut o = offset as i32;

        for _ in 0..num_pixels {
            stream.read_exact(&mut rgbe)?;

            let color =  Reader::rgbe_to_float(rgbe);

            image.set_by_index(o, color);
            o += 1;
        }

        Ok(())
    }

    fn rgbe_to_float(rgbe: [u8; 4]) -> float3 {
        if rgbe[3] > 0 {
            // nonzero pixel
            let f = (((rgbe[3]) as i32 - (128 + 8)) as f32).exp2();

            return float3::new(rgbe[0] as f32 * f, rgbe[1] as f32 * f, rgbe[2] as f32 * f);
        }

        float3::from_scalar(0.0)
    }
}
