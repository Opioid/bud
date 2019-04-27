use super::float3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub org: float3,
    pub dir: float3,
}

impl Ray {
    pub fn new(org: float3, dir: float3) -> Ray {
        Ray { org, dir }
    }
}
