use super::sensor::{Sensor, SensorBase};
use base::math::{int2, float3};
use image::Float3;
use sampler::CameraSample;

pub struct Unfiltered {
    base: SensorBase,
    pixels: Vec<float3>,
}

impl Unfiltered {
    pub fn new(exposure_factor: f32) -> Unfiltered {
        Unfiltered { base: SensorBase::new(exposure_factor), pixels: Vec::new() }
    }
}


impl Sensor for Unfiltered {
    fn resize(&mut self, dimensions: int2) {
        self.base.dimensions = dimensions;
        self.pixels.resize((dimensions.x * dimensions.y) as usize, float3::identity());
    }
    
    fn resolve(&self, target: &mut Float3) {
        for (i, color) in self.pixels.iter().enumerate() {
            target.set_by_index(i as i32, *color);
        }
     }

    fn add_sample(&mut self, sample: &CameraSample, color: &float3) {
        let i = sample.pixel.y * self.base.dimensions.x + sample.pixel.x;
        self.pixels[i as usize] = *color;
    }
}
