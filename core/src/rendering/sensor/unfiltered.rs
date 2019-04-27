use super::sensor::{Sensor, SensorBase};
use base::math::{float3, int2};
use image::Float3;
use sampler::CameraSample;

pub struct Unfiltered {
    base: SensorBase,
    pixels: Vec<float3>,
}

impl Unfiltered {
    pub fn new(exposure_factor: f32) -> Unfiltered {
        Unfiltered {
            base: SensorBase::new(exposure_factor),
            pixels: Vec::new(),
        }
    }
}

impl Sensor for Unfiltered {
    fn resize(&mut self, dimensions: int2) {
        self.base.dimensions = dimensions;
        self.pixels.resize(
            (dimensions.v[0] * dimensions.v[1]) as usize,
            float3::identity(),
        );
    }

    fn resolve(&self, target: &mut Float3) {
        for (i, color) in self.pixels.iter().enumerate() {
            target.set_by_index(i as i32, *color);
        }
    }

    fn add_sample(&mut self, sample: &CameraSample, color: &float3) {
        let i = sample.pixel.v[1] * self.base.dimensions.v[0] + sample.pixel.v[0];
        self.pixels[i as usize] = *color;
    }
}
