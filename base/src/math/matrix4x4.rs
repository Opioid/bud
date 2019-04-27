use super::{float3, float4};

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float4x4 {
    pub r: [float4; 4],
}

impl float4x4 {
    pub fn identity() -> float4x4 {
        float4x4 {
            r: [
                float4::new(1.0, 0.0, 0.0, 0.0),
                float4::new(0.0, 1.0, 0.0, 0.0),
                float4::new(0.0, 0.0, 1.0, 0.0),
                float4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn compose(scale: &float3, origin: &float3) -> float4x4 {
        float4x4 {
            r: [
                float4::new(1.0 * scale.v[0], 0.0 * scale.v[0], 0.0 * scale.v[0], 0.0),
                float4::new(0.0 * scale.v[1], 1.0 * scale.v[1], 0.0 * scale.v[1], 0.0),
                float4::new(0.0 * scale.v[2], 0.0 * scale.v[2], 1.0 * scale.v[2], 0.0),
                float4::new(origin.v[0], origin.v[1], origin.v[2], 1.0),
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

    pub fn transform_point(&self, v: &float3) -> float3 {
        float3::new(
            v.v[0] * self.r[0].v[0]
                + v.v[1] * self.r[1].v[0]
                + v.v[2] * self.r[2].v[0]
                + self.r[3].v[0],
            v.v[0] * self.r[0].v[1]
                + v.v[1] * self.r[1].v[1]
                + v.v[2] * self.r[2].v[1]
                + self.r[3].v[1],
            v.v[0] * self.r[0].v[2]
                + v.v[1] * self.r[1].v[2]
                + v.v[2] * self.r[2].v[2]
                + self.r[3].v[2],
        )
    }
}
