use error::Error;
use serde_json::Value;
use std::io::BufRead;

use take::Take;

pub struct Loader {}

impl Loader {
    pub fn load(stream: &mut BufRead) -> Result<Take, Error> {
        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new("No suitable root object."));
        }

        let root = root.as_object().unwrap();

        let mut take = Take::new();

        for (name, value) in root.iter() {
            match name.as_ref() {
                "start_frame" => take.view.start_frame = value.as_u64().unwrap() as u32,
                "num_frames" => take.view.num_frames = value.as_u64().unwrap() as u32,
                "scene" => take.scene_filename = value.as_str().unwrap().to_string(),
                _ => continue,
            }
        }

        Ok(take)
    }
}
