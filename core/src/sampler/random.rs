use super::{Sampler, SamplerBase};
use base::math::float2;
use base::random;

pub struct Random<'a> {
    base: SamplerBase<'a>,
}

impl<'a> Random<'a> {
 pub fn new(rng: &mut random::Generator) -> Random {
        Random { base: SamplerBase::new(rng) }
 }
}

impl<'a> Sampler for Random<'a> {
    fn generate_sample_2D(&mut self) -> float2 {
        float2::new(self.base.rng.random_float(), self.base.rng.random_float())
    }
}
