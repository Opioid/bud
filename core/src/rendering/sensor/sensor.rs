use base::math::{float4, int2, int4};
use image::Float4;
use sampler::CameraSample;

pub struct SensorBase {
    pub dimensions: int2,
    pub exposure_factor: f32,
}

impl SensorBase {
    pub fn new(dimensions: int2, exposure: f32) -> SensorBase {
        SensorBase { dimensions, exposure_factor: exposure.exp2() }
    }
}

pub trait Sensor {
    fn has_alpha_transparency(&self) -> bool;

    fn resolve(&self, target: &mut Float4);

    fn filter_radius_int(&self) -> i32;

    fn add_sample(&mut self, sample: &CameraSample, color: float4, bounds: int4);
}

pub trait TypedSensor {
    fn new(dimensions: int2, exposure: f32) -> Self;

    fn has_alpha_transparency(&self) -> bool;

    fn resolve(&self, target: &mut Float4);

    fn add_pixel(&mut self, pixel: int2, color: float4, weight: f32);
}
