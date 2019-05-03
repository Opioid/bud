use super::Integrator;
use base::math::float4;
use base::random;
use rendering::Worker;
use sampler::{Random, Sampler};
use scene::prop::Intersection;
use scene::{Ray, Scene};

pub struct Ao {
    sampler: Random,
    scrambler: Random,
}

impl Ao {
    pub fn new() -> Ao {
        Ao {
            sampler: Random::new(),
            scrambler: Random::new(),
        }
    }
}

impl Integrator for Ao {
    fn li(
        &mut self,
        scene: &Scene,
        ray: &mut Ray,
        intersection: &mut Intersection,
        worker: &mut Worker,
    ) -> float4 {
        let r0 = self.sampler.generate_sample_2D(worker.rng());
        let r1 = self.scrambler.generate_sample_2D(worker.rng());

        return float4::new(1.0, 1.0, 1.0, 1.0);
    }
}

pub struct AoFactory {}

impl AoFactory {
    pub fn create() -> Box<dyn Integrator> {
        Box::new(Ao::new())
    }
}
