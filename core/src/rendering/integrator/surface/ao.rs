use super::Integrator;
use base::math::float4;
use rendering::Worker;
use scene::prop::Intersection;
use scene::Ray;

pub struct Ao {}

impl<'a, 'b> Integrator<'a, 'b> for Ao {
    fn li(
        &'b mut self,
        ray: &mut Ray,
        intersection: &mut Intersection<'a, 'b>,
        worker: &mut Worker,
    ) -> float4 {
        return float4::new(1.0, 1.0, 1.0, 1.0);
    }
}

pub struct AoFactory {}

impl<'a, 'b> AoFactory {
    pub fn create() -> Box<dyn Integrator<'a, 'b>> {
        Box::new(Ao {})
    }
}
