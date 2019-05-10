use super::ComposedTransformation;
use base::math::{float3, float3x3, Transformation};

pub struct Entity {
    world_transformation: ComposedTransformation,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { world_transformation: ComposedTransformation::new() }
    }

    pub fn transformation_at(&self, time: u64) -> ComposedTransformation {
        self.world_transformation
    }

    pub fn set_transformation(&mut self, transformation: &Transformation) {
        self.world_transformation.set(transformation);
    }
}
