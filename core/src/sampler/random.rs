use super::{CameraSample, Factory, Sampler};
use base::math::{float2, int2};
use base::random;

pub struct Random {}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}

impl Sampler for Random {
    fn resize(
        &mut self,
        _num_iterations: u32,
        _num_samples_per_iteration: u32,
        _num_dimensions_2d: u32,
        _num_dimensions_1d: u32,
    ) {
    }

    fn start_pixel(&mut self) {}

    fn generate_camera_sample(
        &mut self,
        rng: &mut random::Generator,
        pixel: int2,
        index: u32,
    ) -> CameraSample {
        CameraSample::new(pixel, float2::new(rng.random_float(), rng.random_float()))
    }

    fn generate_sample_2d(&mut self, rng: &mut random::Generator, dimension: u32) -> float2 {
        float2::new(rng.random_float(), rng.random_float())
    }
}

pub struct RandomFactory {}

impl Factory for RandomFactory {
    fn create(&self) -> Box<dyn Sampler> {
        Box::new(Random {})
    }
}
