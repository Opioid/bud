use base::math::float4;
use rendering::Worker;
use scene::prop::Intersection;
use scene::{Ray, Scene};

pub trait Integrator {
    fn prepare(&mut self, num_samples_per_pixel: u32);

    fn start_pixel(&mut self);

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
