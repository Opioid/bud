use super::{Camera, CameraBase};
use base::math::{self, int2};
use sampler::CameraSample;
use scene::Ray;
use rendering::sensor::Sensor;

pub struct Perspective {
    base: CameraBase,
    fov: f32,
}

impl Perspective {
    pub fn new(resolution: int2, sensor: Box<dyn Sensor>) -> Perspective {
        let mut p = Perspective {
            base: CameraBase::new(resolution, sensor),
            fov: math::degrees_to_radians(60.0),
        };

        let dimensions = p.sensor_dimensions();
        p.base.sensor.resize(dimensions);
        
        p
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
}

impl Camera for Perspective {
    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray> {
        Some(Ray::new())
    }

    fn sensor_mut(&mut self) -> &mut dyn Sensor {
        &mut (*self.base.sensor)
    }
    
    fn sensor_dimensions(&self) -> int2 {
        self.base.resolution
    }
}
