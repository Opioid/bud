use math;
use math::{float2, float3};

pub fn sample_disk_concentric(uv: float2) -> float2 {
    let s = 2.0 * uv - 1.0;

    if 0.0 == s.v[0] || 0.0 == s.v[1] {
        return float2::identity();
    }

    let r;
    let theta;

    if s.v[0].abs() > s.v[1].abs() {
        r = s.v[0];
        theta = (math::PI / 4.0) * (s.v[1] / s.v[0]);
    } else {
        r = s.v[1];
        theta = (math::PI / 2.0) - (math::PI / 4.0) * (s.v[0] / s.v[1]);
    }

    let (sin_theta, cos_theta) = theta.sin_cos();

    float2::new(cos_theta * r, sin_theta * r)
}

pub fn sample_hemisphere_cosine(uv: float2) -> float3 {
    let xy = sample_disk_concentric(uv);
    let z = 0.0f32
        .max(1.0 - xy.v[0] * xy.v[0] - xy.v[1] * xy.v[1])
        .sqrt();

    float3::new(xy.v[0], xy.v[1], z)
}
