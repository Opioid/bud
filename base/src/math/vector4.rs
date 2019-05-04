use super::float3;
use std::ops;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float4 {
    pub v: [f32; 4],
}

impl float4 {
    #[inline]
    pub fn identity() -> float4 {
        float4 {
            v: [0.0, 0.0, 0.0, 0.0],
        }
    }

    #[inline]
    pub fn from_3(v: float3, w: f32) -> float4 {
        float4 {
            v: [v.v[0], v.v[1], v.v[2], w],
        }
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> float4 {
        float4 { v: [x, y, z, w] }
    }

    #[inline]
    pub fn xyz(self) -> float3 {
        float3 {
            v: [self.v[0], self.v[1], self.v[2]],
        }
    }
}

impl ops::AddAssign for float4 {
    fn add_assign(&mut self, other: float4) {
        self.v[0] += other.v[0];
        self.v[1] += other.v[1];
        self.v[2] += other.v[2];
        self.v[3] += other.v[3];
    }
}
