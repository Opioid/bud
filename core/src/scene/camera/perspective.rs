use super::{Camera, CameraBase};
use base::math::{self, int2};
use sampler::CameraSample;
use scene::Ray;

pub struct Perspective {
    base: CameraBase,
    fov: f32,
}

impl Perspective {
    pub fn new(resolution: int2) -> Perspective {
        Perspective {
            base: CameraBase::new(resolution),
            fov: math::degrees_to_radians(60.0),
        }
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
}

impl Camera for Perspective {
    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray> {
        Some(Ray::new())
    }
}
