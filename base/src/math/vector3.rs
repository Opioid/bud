use std::ops;

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct float3 {
    pub v: [f32; 3],
}

impl float3 {
    pub fn identity() -> float3 {
        float3 { v: [0.0, 0.0, 0.0] }
    }

    pub fn from_scalar(s: f32) -> float3 {
        float3 { v: [s, s, s] }
    }

    pub fn new(x: f32, y: f32, z: f32) -> float3 {
        float3 { v: [x, y, z] }
    }

    pub fn dot(&self, other: &float3) -> f32 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> float3 {
        *self / self.length()
    }

    pub fn cross(&self, b: &float3) -> float3 {
        float3 {
            v: [
                self.v[1] * b.v[2] - self.v[2] * b.v[1],
                self.v[2] * b.v[0] - self.v[0] * b.v[2],
                self.v[0] * b.v[1] - self.v[1] * b.v[0],
            ],
        }
    }
}

impl ops::Add<float3> for float3 {
    type Output = float3;

    fn add(self, other: float3) -> float3 {
        float3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
            ],
        }
    }
}

impl ops::Sub<float3> for float3 {
    type Output = float3;

    fn sub(self, other: float3) -> float3 {
        float3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

impl ops::Neg for float3 {
    type Output = float3;

    fn neg(self) -> float3 {
        float3 {
            v: [-self.v[0], -self.v[1], -self.v[2]],
        }
    }
}

impl ops::Mul<float3> for f32 {
    type Output = float3;

    fn mul(self, v: float3) -> float3 {
        float3 {
            v: [self * v.v[0], self * v.v[1], self * v.v[2]],
        }
    }
}

impl ops::Div<f32> for float3 {
    type Output = float3;

    fn div(self, s: f32) -> float3 {
        float3 {
            v: [self.v[0] / s, self.v[1] / s, self.v[2] / s],
        }
    }
}
