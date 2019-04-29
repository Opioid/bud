use super::Integrator;
use base::math::float4;
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
    pub fn create() -> Box<dyn Integrator> {
        Box::new(Ao {})
    }
}
