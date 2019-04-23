use super::ComposedTransformation;
use base::math::Transformation;

pub struct Entity {
    world_transformation: ComposedTransformation,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            world_transformation: ComposedTransformation::new(),
        }
    }

    pub fn set_transformation(&mut self, transformation: &Transformation) {
        self.world_transformation.set(transformation);
    }
}
