use serde_json::Value;
use std::io;
use std::io::{BufRead, Error, ErrorKind};

use take::Take;

pub struct Loader {}

impl Loader {
    pub fn load(stream: &mut BufRead) -> io::Result<Take> {
        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new(ErrorKind::Other, "No suitable root object"));
        }

        let root = root.as_object().unwrap();

        let mut take = Take::new();

        for (name, value) in root.iter() {
            if "scene" == name {
                take.scene_filename = value.as_str().unwrap().to_string();
            }
        }

        Ok(take)
    }
}
