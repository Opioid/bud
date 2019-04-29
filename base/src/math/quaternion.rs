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
            let s = (trace + 1.0).sqrt();
            let t3 = s * 0.5;
            let s = 0.5 / s;

            let t0 = (m.r[2].v[1] - m.r[1].v[2]) * s;
            let t1 = (m.r[0].v[2] - m.r[2].v[0]) * s;
            let t2 = (m.r[1].v[0] - m.r[0].v[1]) * s;

            return Quaternion {
                0: float4 {
                    v: [t0, t1, t2, t3],
                },
            };
        } else {
            let i = match m.r[0].v[0] < m.r[1].v[1] {
                true => match m.r[1].v[1] < m.r[2].v[2] {
                    true => 2,
                    false => 1,
                },
                false => match m.r[0].v[0] < m.r[2].v[2] {
                    true => 2,
                    false => 0,
                },
            };

            let j = (i + 1) % 3;
            let k = (i + 2) % 3;

            let mut t = Quaternion::identity();

            let s = (m.r[i].v[i] - m.r[j].v[j] - m.r[k].v[k] + 1.0).sqrt();
            t.0.v[i] = s * 0.5;
            let s = 0.5 / s;

            t.0.v[3] = (m.r[k].v[j] - m.r[j].v[k]) * s;
            t.0.v[j] = (m.r[j].v[i] - m.r[i].v[j]) * s;
            t.0.v[k] = (m.r[k].v[i] - m.r[i].v[k]) * s;

            return t;
        }
    }

    pub fn create_matrix3x3(q: &Quaternion) -> float3x3 {
        let d = q.dot(q);
        let s = 2.0 / d;

        let xs = q.0.v[0] * s;
        let ys = q.0.v[1] * s;
        let zs = q.0.v[2] * s;

        let mut m = float3x3::identity();

        {
            let xx = q.0.v[0] * xs;
            let yy = q.0.v[1] * ys;
            let zz = q.0.v[2] * zs;
            m.r[0].v[0] = 1.0 - (yy + zz);
            m.r[1].v[1] = 1.0 - (xx + zz);
            m.r[2].v[2] = 1.0 - (xx + yy);
        }

        {
            let xy = q.0.v[0] * ys;
            let wz = q.0.v[3] * zs;
            m.r[0].v[1] = xy - wz;
            m.r[1].v[0] = xy + wz;
        }

        {
            let xz = q.0.v[0] * zs;
            let wy = q.0.v[3] * ys;
            m.r[0].v[2] = xz + wy;
            m.r[2].v[0] = xz - wy;
        }

        {
            let yz = q.0.v[1] * zs;
            let wx = q.0.v[3] * xs;
            m.r[1].v[2] = yz - wx;
            m.r[2].v[1] = yz + wx;
        }

        m
    }

    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.0.v[0] * other.0.v[0]
            + self.0.v[1] * other.0.v[1]
            + self.0.v[2] * other.0.v[2]
            + self.0.v[3] * other.0.v[3]
    }
}
