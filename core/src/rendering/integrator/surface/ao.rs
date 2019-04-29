use super::Integrator;
use base::math::float4;
use base::random;
use rendering::Worker;
use scene::prop::Intersection;
use scene::Ray;

pub struct Ao {}

impl Integrator for Ao {
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

impl AoFactory {
    pub fn create(rng: &mut random::Generator) -> Box<dyn Integrator> {
        Box::new(Ao {})
    }
}
