use super::{float3x3, float4};

pub struct Quaternion(pub float4);

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
            0: float4 {
                v: [0.0, 0.0, 0.0, 1.0],
            },
        }
    }

    pub fn from_matrix(m: &float3x3) -> Quaternion {
        let trace = m.r[0].v[0] + m.r[1].v[1] + m.r[2].v[2];

        if trace > 0.0 {
            return Quaternion::identity();
        } else {
            return Quaternion::identity();
        }
    }
}
