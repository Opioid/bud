use base::math::{float4, int2};
use image::Float3;
use sampler::CameraSample;

pub struct SensorBase {
    pub dimensions: int2,
    pub exposure_factor: f32,
}

impl SensorBase {
    pub fn new(exposure_factor: f32) -> SensorBase {
        SensorBase {
            dimensions: int2::identity(),
            exposure_factor,
        }
    }
}

pub trait Sensor {
    fn resize(&mut self, dimensions: int2);

    fn resolve(&self, target: &mut Float3);

    fn add_sample(&mut self, sample: &CameraSample, color: float4);
}
