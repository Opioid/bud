use super::Sampler;
use base::math::float2;
use base::random;

pub struct Random {}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}

impl Sampler for Random {
    fn generate_sample_2D(&mut self, rng: &mut random::Generator) -> float2 {
        float2::new(rng.random_float(), rng.random_float())
    }
}
