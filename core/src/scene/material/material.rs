use base::math::float3;
use rendering::Worker;
use resource::Identifiable;

pub struct MaterialBase {
    pub stuff: u32,
}

pub trait Material {
    fn evaluate_radiance(&self, wi: float3, worker: &Worker) -> float3;
}
