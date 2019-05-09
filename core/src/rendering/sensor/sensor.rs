use base::math::{float4, int2};
use image::Float4;
use sampler::CameraSample;

pub struct SensorBase {
    pub dimensions: int2,
    pub exposure_factor: f32,
}

impl SensorBase {
    pub fn new(exposure: f32) -> SensorBase {
        SensorBase {
            dimensions: int2::identity(),
            exposure_factor: exposure.exp2(),
        }
    }
}

pub trait Sensor {
    fn resize(&mut self, dimensions: int2);

    fn resolve(&self, target: &mut Float4);

    fn add_sample(&mut self, sample: &CameraSample, color: float4);
}

pub trait TypedSensor {
    //   type Type;

    fn new(exposure: f32) -> Self;

    fn has_alpha_transparency(&self) -> bool;

    fn resize(&mut self, dimensions: int2);

    fn resolve(&self, target: &mut Float4);

    fn add_pixel(&mut self, pixel: int2, color: float4, weight: f32);
}
