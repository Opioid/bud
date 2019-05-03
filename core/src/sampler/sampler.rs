use base::math::float2;
use base::random;

pub trait Sampler {
    fn generate_sample_2d(&mut self, rng: &mut random::Generator) -> float2;
}
