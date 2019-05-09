use super::math::{byte3, float3};

pub fn float_to_unorm(x: f32) -> u8 {
    (x * 255.0 + 0.5) as u8
}

pub fn float_to_unorm_3(c: float3) -> byte3 {
    byte3::new(
        float_to_unorm(c.v[0]),
        float_to_unorm(c.v[1]),
        float_to_unorm(c.v[2]),
    )
}
