use super::sensor::{Sensor, SensorBase};
use base::math::{float4, int2};
use image::Float3;
use sampler::CameraSample;

pub struct Unfiltered {
    base: SensorBase,
    pixels: Vec<float4>,
}

impl Unfiltered {
    pub fn new(exposure: f32) -> Unfiltered {
        Unfiltered {
            base: SensorBase::new(exposure),
            pixels: Vec::new(),
        }
    }
}

impl Sensor for Unfiltered {
    fn resize(&mut self, dimensions: int2) {
        self.base.dimensions = dimensions;
        self.pixels.resize(
            (dimensions.v[0] * dimensions.v[1]) as usize,
            float4::identity(),
        );
    }

    fn resolve(&self, target: &mut Float3) {
        let exposure_factor = self.base.exposure_factor;

        for (i, pixel) in self.pixels.iter().enumerate() {
            let color = pixel.xyz() / pixel.v[3];
            target.set_by_index(i as i32, exposure_factor * color);
        }
    }

    fn add_sample(&mut self, sample: &CameraSample, color: float4) {
        let i = sample.pixel.v[1] * self.base.dimensions.v[0] + sample.pixel.v[0];

        let value = float4::from_3(color.xyz(), 1.0);

        unsafe {
            *self.pixels.get_unchecked_mut(i as usize) += value;
        }
    }
}
