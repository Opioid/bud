use super::Integrator;
use base::math::float4;
use base::random;
use rendering::Worker;
use scene::prop::Intersection;
use scene::Ray;
use sampler::Random;

pub struct Ao<'a> {
    sampler: Random<'a>,
    scrambler: Random<'a>,
}

impl<'a> Ao<'a> {
    pub fn new(rng: &mut random::Generator) -> Ao {
        Ao { sampler: Random::new(rng), scrambler: Random::new(rng) }
    }
}

impl<'a> Integrator for Ao<'a> {
    fn li(
        &mut self,
        ray: &mut Ray,
        intersection: &mut Intersection,
        worker: &mut Worker,
    ) -> float4 {
        
        
        return float4::new(1.0, 1.0, 1.0, 1.0);
    }
}

pub struct AoFactory {}

impl<'a> AoFactory {
    pub fn create(rng: &'a mut random::Generator) -> Box<dyn Integrator + 'a> {
        Box::new(Ao::new(rng))
    }
}
