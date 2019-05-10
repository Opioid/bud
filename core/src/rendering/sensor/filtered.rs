use super::sensor::{Sensor, TypedSensor};
use base::math::{float4, int2};
use image::Float4;
use sampler::CameraSample;

pub struct Filtered1p0<T> {
    base: T,
}

impl<T> Filtered1p0<T>
where
    T: TypedSensor,
{
    pub fn new(exposure: f32) -> Filtered1p0<T> {
        Filtered1p0 {
            base: T::new(exposure),
        }
    }
}

impl<T> Sensor for Filtered1p0<T>
where
    T: TypedSensor,
{
    fn has_alpha_transparency(&self) -> bool {
        self.base.has_alpha_transparency()
    }

    fn resize(&mut self, dimensions: int2) {
        self.base.resize(dimensions)
    }

    fn resolve(&self, target: &mut Float4) {
        self.base.resolve(target)
    }

    fn add_sample(&mut self, sample: &CameraSample, color: float4) {
        add_weighted(&mut self.base, sample.pixel, color, 1.0)
    }
}

fn add_weighted<T: TypedSensor>(sensor: &mut T, pixel: int2, color: float4, weight: f32) {
    sensor.add_pixel(pixel, color, weight)
}
