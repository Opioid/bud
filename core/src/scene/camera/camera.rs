use base::math::int2;
use rendering::sensor::Sensor;
use sampler::CameraSample;
use scene::entity::Entity;
use scene::Ray;
use serde_json::Value;

pub struct CameraBase {
    pub entity: Entity,
    pub resolution: int2,
    pub sensor: Box<dyn Sensor>,
}

impl CameraBase {
    pub fn new(resolution: int2, sensor: Box<dyn Sensor>) -> CameraBase {
        CameraBase {
            entity: Entity::new(),
            resolution,
            sensor,
        }
    }

    pub fn set_parameters(camera: &mut impl Camera, parameters_value: &Value) {
        if let Value::Object(parameters_value) = parameters_value {
            for (name, value) in parameters_value {
                camera.set_parameter(name, value);
            }
        }
    }
}

pub trait Camera {
    fn update(&mut self) {
        self.on_update();
    }

    fn on_update(&mut self);

    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray>;

    fn sensor_mut(&mut self) -> &mut dyn Sensor;

    fn sensor_dimensions(&self) -> int2;

    fn set_parameter(&mut self, name: &str, value: &Value);
}
