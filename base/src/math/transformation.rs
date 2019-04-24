use super::float3;

pub struct Transformation {
    pub position: float3,
    pub scale: float3,
}

impl Transformation {
    pub fn identity() -> Transformation {
        Transformation {
            position: float3::identity(),
            scale: float3::from_scalar(1.0),
        }
    }
}
