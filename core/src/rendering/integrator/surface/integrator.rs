use base::math::float4;
use rendering::Worker;
use scene::prop::Intersection;
use scene::{Ray, Scene};

pub trait Integrator {
    fn li(
        &mut self,
        scene: &Scene,
        ray: &mut Ray,
        intersection: &mut Intersection,
        worker: &mut Worker,
    ) -> float4;
}

pub trait Factory {
    fn create(&self) -> Box<dyn Integrator>;
}
