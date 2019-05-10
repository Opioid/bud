use super::sensor::{Sensor, TypedSensor};
use base::math::{float4, int2, int4};
use image::Float4;
use sampler::CameraSample;

pub struct Unfiltered<T> {
    base: T,
}

impl<T> Unfiltered<T>
where
    T: TypedSensor,
{
    pub fn new(dimensions: int2, exposure: f32) -> Unfiltered<T> {
        Unfiltered { base: T::new(dimensions, exposure) }
    }
}

impl<T> Sensor for Unfiltered<T>
where
    T: TypedSensor,
{
    fn has_alpha_transparency(&self) -> bool {
        self.base.has_alpha_transparency()
    }

    fn resolve(&self, target: &mut Float4) {
        self.base.resolve(target)
    }

    fn filter_radius_int(&self) -> i32 {
        0
    }

    fn add_sample(&mut self, sample: &CameraSample, color: float4, bounds: int4) {
        self.base.add_pixel(bounds.xy() + sample.pixel, color, 1.0)
    }
}
