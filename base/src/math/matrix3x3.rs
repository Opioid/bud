use super::float3;
use std::ops;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float3x3 {
    pub r: [float3; 3],
}

impl float3x3 {
    pub fn identity() -> float3x3 {
        float3x3 {
            r: [
                float3::new(1.0, 0.0, 0.0),
                float3::new(0.0, 1.0, 0.0),
                float3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn rotation_x(a: f32) -> float3x3 {
        let c = a.cos();
        let s = a.sin();

        float3x3 {
            r: [
                float3::new(1.0, 0.0, 0.0),
                float3::new(0.0, c, -s),
                float3::new(0.0, s, c),
            ],
        }
    }

    pub fn rotation_y(a: f32) -> float3x3 {
        let c = a.cos();
        let s = a.sin();

        float3x3 {
            r: [
                float3::new(c, 0.0, s),
                float3::new(0.0, 1.0, 0.0),
                float3::new(-s, 0.0, c),
            ],
        }
    }

    pub fn rotation_z(a: f32) -> float3x3 {
        let c = a.cos();
        let s = a.sin();

        float3x3 {
            r: [
                float3::new(c, -s, 0.0),
                float3::new(s, c, 0.0),
                float3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn transform_vector(&self, v: &float3) -> float3 {
        float3::new(
            v.v[0] * self.r[0].v[0] + v.v[1] * self.r[1].v[0] + v.v[2] * self.r[2].v[0],
            v.v[0] * self.r[0].v[1] + v.v[1] * self.r[1].v[1] + v.v[2] * self.r[2].v[1],
            v.v[0] * self.r[0].v[2] + v.v[1] * self.r[1].v[2] + v.v[2] * self.r[2].v[2],
        )
    }
}

impl ops::Mul<float3x3> for float3x3 {
    type Output = float3x3;
    fn mul(self, other: float3x3) -> float3x3 {
        float3x3 {
            r: [
                float3::new(
                    self.r[0].v[0] * other.r[0].v[0]
                        + self.r[0].v[1] * other.r[1].v[0]
                        + self.r[0].v[2] * other.r[2].v[0],
                    self.r[0].v[0] * other.r[0].v[1]
                        + self.r[0].v[1] * other.r[1].v[1]
                        + self.r[0].v[2] * other.r[2].v[1],
                    self.r[0].v[0] * other.r[0].v[2]
                        + self.r[0].v[1] * other.r[1].v[2]
                        + self.r[0].v[2] * other.r[2].v[2],
                ),
                float3::new(
                    self.r[1].v[0] * other.r[0].v[0]
                        + self.r[1].v[1] * other.r[1].v[0]
                        + self.r[1].v[2] * other.r[2].v[0],
                    self.r[1].v[0] * other.r[0].v[1]
                        + self.r[1].v[1] * other.r[1].v[1]
                        + self.r[1].v[2] * other.r[2].v[1],
                    self.r[1].v[0] * other.r[0].v[2]
                        + self.r[1].v[1] * other.r[1].v[2]
                        + self.r[1].v[2] * other.r[2].v[2],
                ),
                float3::new(
                    self.r[2].v[0] * other.r[0].v[0]
                        + self.r[2].v[1] * other.r[1].v[0]
                        + self.r[2].v[2] * other.r[2].v[0],
                    self.r[2].v[0] * other.r[0].v[1]
                        + self.r[2].v[1] * other.r[1].v[1]
                        + self.r[2].v[2] * other.r[2].v[1],
                    self.r[2].v[0] * other.r[0].v[2]
                        + self.r[2].v[1] * other.r[1].v[2]
                        + self.r[2].v[2] * other.r[2].v[2],
                ),
            ],
        }
    }
}
