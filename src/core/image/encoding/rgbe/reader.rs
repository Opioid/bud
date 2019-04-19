use std::io;
use std::io::{BufRead, Error, ErrorKind};

use crate::base::math::vector2::int2;
// use crate::base::math::vector3::float3;
use crate::core::image::Float3;

pub struct Reader {}

struct Header {
    width: i32,
    height: i32,
}

impl Reader {
    pub fn read(stream: &mut BufRead) -> io::Result<Float3> {
        let header = Reader::read_header(stream)?;

        let mut image = Float3::new(int2::new(header.width, header.height));

        Reader::read_pixels_rle(&mut image)?;

        Ok(image)
    }

    fn read_header(stream: &mut BufRead) -> io::Result<Header> {
        let mut line = String::new();

        stream.read_line(&mut line)?;

        if &line.as_bytes()[..2] != "#?".as_bytes() {
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

        if let Some(dimensions) = Reader::parse_size(&line) {
            return Ok(Header {
                width: dimensions.x,
                height: dimensions.y,
            });
        }

        Err(Error::new(ErrorKind::Other, "Missing image size specifier"))
    }

    fn parse_size(line: &str) -> Option<int2> {
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

        Some(int2::new(width.unwrap() as i32, height.unwrap() as i32))
    }

    fn read_pixels_rle(_image: &mut Float3) -> io::Result<()> {
        Ok(())
    }
}
