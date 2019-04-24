use error::Error;
use serde_json::Value;
use std::io::BufRead;

use base::math::{int2, Transformation};
use json;
use scene::camera::{Camera, Perspective};
use take::Take;

pub struct Loader {}

impl Loader {
    pub fn load(stream: &mut BufRead) -> Result<Take, Error> {
        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new("No suitable root object."));
        }

        let root = root.as_object().unwrap();

        let mut camera = None;
        let mut start_frame = 0u32;
        let mut num_frames = 1u32;
        let mut scene_filename = String::new();

        for (name, value) in root.iter() {
            match name.as_ref() {
                "camera" => camera = Loader::load_camera(value),
                "start_frame" => start_frame = value.as_u64().unwrap() as u32,
                "num_frames" => num_frames = value.as_u64().unwrap() as u32,
                "scene" => scene_filename = value.as_str().unwrap().to_string(),
                _ => continue,
            }
        }

        if camera.is_none() {
            return Err(Error::new("No camera."));
        }

        let mut take = Take::new(camera.unwrap());

        take.view.start_frame = start_frame;
        take.view.num_frames = num_frames;
        take.scene_filename = scene_filename;

        Ok(take)
    }

    fn load_camera(camera_value: &serde_json::Value) -> Option<Box<dyn Camera>> {
        if !camera_value.is_object() {
            return None;
        }

        let camera_value = camera_value.as_object().unwrap();

        let (type_name, type_value) = camera_value.iter().next().unwrap();

        println!("{}", type_name);

        if !type_value.is_object() {
            return None;
        }

        let type_value = type_value.as_object().unwrap();

        let mut transformation = Transformation::identity();

        let mut sensor_value = None;

        for (name, value) in type_value.iter() {
            match name.as_ref() {
                "sensor" => sensor_value = Some(value),
                "transformation" => json::read_transformation(value, &mut transformation),
                _ => continue,
            }
        }

        let mut resolution = int2::identity();
        match sensor_value {
            Some(sensor_value) => {
                resolution = json::read_int2_from(sensor_value, "resolution", int2::new(0, 0));
            }
            _ => return None,
        }

        if int2::identity() == resolution {
            return None;
        }

        Some(Box::new(Perspective::new(resolution)))
    }
}
