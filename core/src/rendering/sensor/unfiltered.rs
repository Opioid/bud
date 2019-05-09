use super::sensor::{Sensor, TypedSensor};
use base::math::{float4, int2};
use image::Float4;
use sampler::CameraSample;

pub struct Unfiltered<T> {
    base: T,
}

impl<T> Unfiltered<T>
where
    T: TypedSensor,
{
    pub fn new(exposure: f32) -> Unfiltered<T> {
        Unfiltered {
            base: T::new(exposure),
        }
    }
}

impl<T> Sensor for Unfiltered<T>
where
    T: TypedSensor,
{
    fn resize(&mut self, dimensions: int2) {
        self.base.resize(dimensions)
    }

    fn resolve(&self, target: &mut Float4) {
        self.base.resolve(target)
    }

    fn add_sample(&mut self, sample: &CameraSample, color: float4) {
        self.base.add_pixel(sample.pixel, color, 1.0)
    }
}
