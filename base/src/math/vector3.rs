use std::ops;

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl float3 {
    pub fn identity() -> float3 {
        float3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_scalar(s: f32) -> float3 {
        float3 { x: s, y: s, z: s }
    }

    pub fn new(x: f32, y: f32, z: f32) -> float3 {
        float3 { x, y, z }
    }
}

impl ops::Add<float3> for float3 {
    type Output = float3;

    fn add(self, other: float3) -> float3 {
        float3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl ops::Sub<float3> for float3 {
    type Output = float3;

    fn sub(self, other: float3) -> float3 {
        float3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl ops::Div<f32> for float3 {
    type Output = float3;

    fn div(self, s: f32) -> float3 {
        float3{x: self.x / s, y: self.y / s, z: self.z / s}
    }
}
