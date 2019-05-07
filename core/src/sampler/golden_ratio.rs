use super::{CameraSample, Factory, Sampler, SamplerBase};
use base::math::{self, float2, int2};
use base::random;

pub struct GoldenRatio {
    base: SamplerBase,
    samples_2d: Vec<float2>,
    samples_1d: Vec<f32>,
}

impl GoldenRatio {
    pub fn new() -> GoldenRatio {
        GoldenRatio {
            base: SamplerBase::new(),
            samples_2d: Vec::new(),
            samples_1d: Vec::new(),
        }
    }

    fn generate_2d(&mut self, rng: &mut random::Generator, dimension: u32) {
        let r = float2::new(rng.random_float(), rng.random_float());

        let begin = (dimension * self.base.num_samples) as usize;
        let end = begin + self.base.num_samples as usize;

        let slice = &mut self.samples_2d[begin..end];

        math::golden_ratio_2d(slice, r);
        random::biased_shuffle(slice, rng);
    }

    fn generate_1d(&mut self, rng: &mut random::Generator, dimension: u32) {
        let r = rng.random_float();

        let begin = (dimension * self.base.num_samples) as usize;
        let end = begin + self.base.num_samples as usize;

        let slice = &mut self.samples_1d[begin..end];

        math::golden_ratio_1d(slice, r);
        random::biased_shuffle(slice, rng);
    }
}

impl Sampler for GoldenRatio {
    fn resize(
        &mut self,
        num_iterations: u32,
        num_samples_per_iteration: u32,
        num_dimensions_2d: u32,
        num_dimensions_1d: u32,
    ) {
        self.base.resize(
            num_iterations,
            num_samples_per_iteration,
            num_dimensions_2d,
            num_dimensions_1d,
        );

        self.samples_2d.resize(
            (self.base.num_samples * num_dimensions_2d) as usize,
            float2::identity(),
        );

        self.samples_1d
            .resize((self.base.num_samples * num_dimensions_1d) as usize, 0.0);
    }

    fn start_pixel(&mut self) {
        self.base.start_pixel();
    }

    fn generate_camera_sample(
        &mut self,
        rng: &mut random::Generator,
        pixel: int2,
        index: u32,
    ) -> CameraSample {
        if 0 == index {
            self.generate_2d(rng, 0);
            self.generate_2d(rng, 1);
            self.generate_1d(rng, 0);
        }

        CameraSample::new(
            pixel,
            self.samples_2d[index as usize],
            self.samples_2d[(self.base.num_samples + index) as usize],
            self.samples_1d[index as usize],
        )
    }

    fn generate_sample_2d(&mut self, rng: &mut random::Generator, dimension: u32) -> float2 {
        let current;
        unsafe {
            let current_ref = self
                .base
                .current_sample_2d
                .get_unchecked_mut(dimension as usize);
            current = *current_ref;
            *current_ref += 1;
        }

        if 0 == current {
            self.generate_2d(rng, dimension);
        }

        self.samples_2d[(dimension * self.base.num_samples + current) as usize]
    }

    fn generate_sample_1d(&mut self, rng: &mut random::Generator, dimension: u32) -> f32 {
        let current;
        unsafe {
            let current_ref = self
                .base
                .current_sample_1d
                .get_unchecked_mut(dimension as usize);
            current = *current_ref;
            *current_ref += 1;
        }

        if 0 == current {
            self.generate_1d(rng, dimension);
        }

        self.samples_1d[(dimension * self.base.num_samples + current) as usize]
    }
}

pub struct GoldenRatioFactory {}

impl Factory for GoldenRatioFactory {
    fn create(&self) -> Box<dyn Sampler> {
        Box::new(GoldenRatio::new())
    }
}
