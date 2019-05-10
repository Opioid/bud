use base::math::int2;
use rendering::sensor::Sensor;
use sampler::CameraSample;
use scene::entity::Entity;
use scene::{self, Ray};
use serde_json::Value;

pub struct CameraBase {
    pub entity: Entity,
    pub resolution: int2,
    pub sensor: Box<dyn Sensor>,

    frame_step: u64,
    frame_duration: u64,
}

impl CameraBase {
    pub fn new(resolution: int2, sensor: Box<dyn Sensor>) -> CameraBase {
        let frame_step = scene::UNITS_PER_SECOND / 60;

        CameraBase {
            entity: Entity::new(),
            resolution,
            sensor,
            frame_step,
            frame_duration: frame_step,
        }
    }

    pub fn set_parameters(camera: &mut impl Camera, parameters_value: &Value) {
        if let Value::Object(parameters_value) = parameters_value {
            for (name, value) in parameters_value {
                camera.set_parameter(name, value);
            }
        }
    }

    pub fn absolute_time(&self, frame: u32, frame_delta: f32) -> u64 {
        let delta = frame_delta as f64;
        let duration = self.frame_duration as f64;

        let fdi = (delta * duration + 0.5) as u64;

        frame as u64 * self.frame_step + fdi
    }
}

pub trait Camera {
    fn update(&mut self) {
        self.on_update();
    }

    fn on_update(&mut self);

    fn generate_ray(&self, sample: &CameraSample, frame: u32) -> Option<Ray>;

    fn sensor(&self) -> &dyn Sensor;

    fn sensor_mut(&mut self) -> &mut dyn Sensor;

    fn sensor_dimensions(&self) -> int2;

    fn set_parameter(&mut self, name: &str, value: &Value);
}
