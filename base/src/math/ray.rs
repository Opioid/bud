use super::float3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub org: float3,
    pub dir: float3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            org: float3::identity(),
            dir: float3::new(0.0, 0.0, 1.0),
        }
    }
}
