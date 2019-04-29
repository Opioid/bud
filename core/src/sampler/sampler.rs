use base::math::float2;
use base::random;

pub struct SamplerBase<'a> {
    pub rng: &'a mut random::Generator,
}

impl<'a> SamplerBase<'a> {
    pub fn new(rng: &mut random::Generator) -> SamplerBase {
        SamplerBase { rng }
    }
}

pub trait Sampler {
    fn generate_sample_2D(&mut self) -> float2;
} 
