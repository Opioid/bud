use base::math::float4;
use rendering::Worker;
use scene::prop::Intersection;
use scene::Ray;

pub trait Integrator {
    fn li(&mut self, ray: &mut Ray, intersection: &mut Intersection, worker: &mut Worker)
        -> float4;
}
