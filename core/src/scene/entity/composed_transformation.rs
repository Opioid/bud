use base::math::float3;
use base::math::Transformation;

pub struct ComposedTransformation {
    position: float3,
}

impl ComposedTransformation {
    pub fn new() -> ComposedTransformation {
        ComposedTransformation {
            position: float3::from_scalar(0.0),
        }
    }

    pub fn set(&mut self, transformation: &Transformation) {
        self.position = transformation.position;
    }
}
