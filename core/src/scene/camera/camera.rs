use base::math::int2;
use sampler::CameraSample;
use scene::entity::Entity;
use scene::Ray;

pub struct CameraBase {
    pub entity: Entity,
    pub resolution: int2,
}

impl CameraBase {
    pub fn new(resolution: int2) -> CameraBase {
        CameraBase {
            entity: Entity::new(),
            resolution,
        }
    }
}

pub trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray>;
}
