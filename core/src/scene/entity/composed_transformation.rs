use base::math::Transformation;
use base::math::{float3, float4x4};

#[derive(Copy, Clone)]
pub struct ComposedTransformation {
    pub world_to_object: float4x4,
    pub object_to_world: float4x4,
    pub position: float3,
    pub scale: float3,
}

impl ComposedTransformation {
    pub fn new() -> ComposedTransformation {
        ComposedTransformation {
            world_to_object: float4x4::identity(),
            object_to_world: float4x4::identity(),
            position: float3::identity(),
            scale: float3::from_scalar(1.0),
        }
    }

    pub fn set(&mut self, t: &Transformation) {
        let otw = float4x4::compose(&t.scale, &t.position);

        self.object_to_world = otw;

        self.position = t.position;
        self.scale = t.scale;
    }
}
