use error::Error;
use serde_json::Value;
use std::io::BufRead;

use base::math::{int2, Transformation};
use exporting;
use image;
use json;
use rendering::sensor::{Sensor, Unfiltered};
use scene::camera::{Camera, CameraBase, Perspective};
use take::{Take, View};

pub struct Loader {}

impl Loader {
    pub fn load(stream: &mut BufRead) -> Result<Take, Error> {
        let root: Value = serde_json::from_reader(stream)?;
        let root = match root {
            Value::Object(root) => root,
            _ => return Err(Error::new("No suitable root object.")),
        };

        let mut camera = None;
        let mut export_value = None;
        let mut start_frame = 0u32;
        let mut num_frames = 1u32;
        let mut scene_filename = String::new();

        for (name, value) in root.iter() {
            match name.as_ref() {
                "camera" => camera = Loader::load_camera(value),
                "export" => export_value = Some(value),
                "start_frame" => start_frame = value.as_u64().unwrap() as u32,
                "num_frames" => num_frames = value.as_u64().unwrap() as u32,
                "scene" => scene_filename = value.as_str().unwrap().to_string(),
                _ => (),
            }
        }

        if camera.is_none() {
            return Err(Error::new("No camera."));
        }

        let mut take = Take::new(camera.unwrap());

        take.view.start_frame = start_frame;
        take.view.num_frames = num_frames;
        take.scene_filename = scene_filename;

        if let Some(export_value) = export_value {
            take.exporters = Loader::load_exporters(export_value, &mut take.view);
        }

        if take.exporters.is_empty() {
            let writer = image::encoding::rgbe::Writer {};
            take.exporters.push(Box::new(exporting::ImageSequence::new(
                "output_".to_string(),
                writer,
            )));
        }

        Ok(take)
    }

    fn load_camera(camera_value: &Value) -> Option<Box<dyn Camera>> {
        let camera_value = camera_value.as_object()?;

        let (type_name, type_value) = camera_value.iter().next().unwrap();

        println!("{}", type_name);

        let type_value = type_value.as_object()?;

        let mut transformation = Transformation::identity();

        let mut sensor = None;

        let mut parameters_value = None;

        for (name, value) in type_value.iter() {
            match name.as_ref() {
                "sensor" => sensor = Loader::load_sensor(value),
                "transformation" => json::read_transformation(value, &mut transformation),
                "parameters" => parameters_value = Some(value),
                _ => (),
            }
        }

        let (resolution, sensor) = sensor?;

        if int2::identity() == resolution {
            return None;
        }

        let mut camera = Box::new(Perspective::new(resolution, sensor));

        if let Some(parameters_value) = parameters_value {
            CameraBase::set_parameters(&mut (*camera), parameters_value);
        }

        camera.base.entity.set_transformation(&transformation);

        Some(camera)
    }

    fn load_sensor(sensor_value: &Value) -> Option<(int2, Box<dyn Sensor>)> {
        let sensor_value = sensor_value.as_object()?;

        let mut resolution = int2::identity();
        let mut exposure = 0.0;
        let mut alpha_transparency;

        for (name, value) in sensor_value.iter() {
            match name.as_ref() {
                "resolution" => resolution = json::read_int2(value),
                "exposure" => exposure = json::read_float(value),
                "alpha_transparency" => alpha_transparency = value.as_bool().unwrap(),
                _ => (),
            }
        }

        Some((resolution, Box::new(Unfiltered::new(exposure))))
    }

    fn load_exporters(export_value: &Value, view: &mut View) -> Vec<Box<dyn exporting::Sink>> {
        let mut exporters = Vec::new();

        let export_value = match export_value {
            Value::Object(export_value) => export_value,
            _ => return exporters,
        };

        for (name, value) in export_value.iter() {
            match name.as_ref() {
                "Image" => {
                    let format = json::read_string_from(value, "format", "PNG");

                    match format {
                        "RGBE" => {
                            let writer = image::encoding::rgbe::Writer {};
                            exporters.push(Box::new(exporting::ImageSequence::new(
                                "output_".to_string(),
                                writer,
                            )));
                        }
                        _ => {
                            let writer = image::encoding::png::Writer {};
                            exporters.push(Box::new(exporting::ImageSequence::new(
                                "output_".to_string(),
                                writer,
                            )));
                        }
                    }
                }
                "Statistics" => exporters.push(Box::new(exporting::Statistics {})),
                _ => (),
            }
        }

        exporters
    }
}
