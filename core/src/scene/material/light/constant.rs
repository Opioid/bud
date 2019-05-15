use base::math::float3;
use rendering::Worker;
use scene::material::Material;

pub struct Constant {
    emittance: float3,
}

impl Constant {
    pub fn new() -> Constant {
        Constant { emittance: float3::identity() }
    }

    pub fn emittance(&mut self) -> &mut float3 {
        &mut self.emittance
    }
}

impl Material for Constant {
    fn evaluate_radiance(&self, wi: float3, worker: &Worker) -> float3 {
        self.emittance
    }
}
