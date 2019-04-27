use super::{float3, Quaternion};

pub struct Transformation {
    pub position: float3,
    pub scale: float3,
    pub rotation: Quaternion,
}

impl Transformation {
    pub fn identity() -> Transformation {
        Transformation {
            position: float3::identity(),
            scale: float3::from_scalar(1.0),
            rotation: Quaternion::identity(),
        }
    }
}
