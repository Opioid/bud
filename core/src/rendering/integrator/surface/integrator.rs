use base::math::float4;
use rendering::Worker;
use scene::prop::Intersection;
use scene::Ray;

pub trait Integrator<'a, 'b> {
    fn li(
        &'b mut self,
        ray: &mut Ray,
        intersection: &mut Intersection<'a, 'b>,
        worker: &mut Worker,
    ) -> float4;
}
