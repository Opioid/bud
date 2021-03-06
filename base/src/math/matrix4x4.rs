use super::{float3, float3x3, float4};

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float4x4 {
    pub r: [float4; 4],
}

impl float4x4 {
    #[inline]
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

    #[inline]
    pub fn compose(basis: &float3x3, scale: float3, origin: float3) -> float4x4 {
        float4x4 {
            r: [
                float4::new(
                    basis.r[0].v[0] * scale.v[0],
                    basis.r[0].v[1] * scale.v[0],
                    basis.r[0].v[2] * scale.v[0],
                    0.0,
                ),
                float4::new(
                    basis.r[1].v[0] * scale.v[1],
                    basis.r[1].v[1] * scale.v[1],
                    basis.r[1].v[2] * scale.v[1],
                    0.0,
                ),
                float4::new(
                    basis.r[2].v[0] * scale.v[2],
                    basis.r[2].v[1] * scale.v[2],
                    basis.r[2].v[2] * scale.v[2],
                    0.0,
                ),
                float4::new(origin.v[0], origin.v[1], origin.v[2], 1.0),
            ],
        }
    }

    #[inline]
    pub fn transform_vector(self, v: float3) -> float3 {
        float3::new(
            v.v[0] * self.r[0].v[0] + v.v[1] * self.r[1].v[0] + v.v[2] * self.r[2].v[0],
            v.v[0] * self.r[0].v[1] + v.v[1] * self.r[1].v[1] + v.v[2] * self.r[2].v[1],
            v.v[0] * self.r[0].v[2] + v.v[1] * self.r[1].v[2] + v.v[2] * self.r[2].v[2],
        )
    }

    #[inline]
    pub fn transform_point(self, v: float3) -> float3 {
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
