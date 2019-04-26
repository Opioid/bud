use base::math::int2;
use rendering::sensor::Sensor;
use sampler::CameraSample;
use scene::entity::Entity;
use scene::Ray;

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
}

pub trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray>;

    fn sensor_mut(&mut self) -> &mut dyn Sensor;
    
    fn sensor_dimensions(&self) -> int2;
}
