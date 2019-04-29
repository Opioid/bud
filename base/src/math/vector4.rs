use super::float3;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float4 {
    pub v: [f32; 4],
}

impl float4 {
    pub fn identity() -> float4 {
        float4 {
            v: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> float4 {
        float4 { v: [x, y, z, w] }
    }

    pub fn xyz(&self) -> float3 {
        float3 {
            v: [self.v[0], self.v[1], self.v[2]],
        }
    }
}
