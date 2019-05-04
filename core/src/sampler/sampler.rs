use super::CameraSample;
use base::math::{float2, int2};
use base::random;

pub struct SamplerBase {
    pub num_samples: u32,
    pub num_samples_per_iteration: u32,

    pub current_sample_2d: Vec<u32>,
    pub current_sample_1d: Vec<u32>,
}

impl SamplerBase {
    pub fn new() -> SamplerBase {
        SamplerBase {
            num_samples: 0,
            num_samples_per_iteration: 0,
            current_sample_2d: Vec::new(),
            current_sample_1d: Vec::new(),
        }
    }

    pub fn resize(
        &mut self,
        num_iterations: u32,
        num_samples_per_iteration: u32,
        num_dimensions_2d: u32,
        num_dimensions_1d: u32,
    ) {
        self.num_samples = num_iterations * num_samples_per_iteration;
        self.num_samples_per_iteration = num_samples_per_iteration;

        self.current_sample_2d.resize(num_dimensions_2d as usize, 0);
        self.current_sample_1d.resize(num_dimensions_1d as usize, 0);
    }

    pub fn start_pixel(&mut self) {
        for c in self.current_sample_2d.iter_mut() {
            *c = 0;
        }

        for c in self.current_sample_1d.iter_mut() {
            *c = 0;
        }
    }
}

pub trait Sampler {
    fn resize(
        &mut self,
        num_iterations: u32,
        num_samples_per_iteration: u32,
        num_dimensions_2d: u32,
        num_dimensions_1d: u32,
    );

    fn start_pixel(&mut self);

    fn generate_camera_sample(
        &mut self,
        rng: &mut random::Generator,
        pixel: int2,
        index: u32,
    ) -> CameraSample;

    fn generate_sample_2d(&mut self, rng: &mut random::Generator, dimension: u32) -> float2;
}

pub trait Factory {
    fn create(&self) -> Box<dyn Sampler>;
}
