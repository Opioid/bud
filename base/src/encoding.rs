use super::math::{byte3, byte4, float3, float4};

pub fn float_to_unorm(x: f32) -> u8 {
    (x * 255.0 + 0.5) as u8
}

pub fn float_to_unorm_3(c: float3) -> byte3 {
    byte3::new(float_to_unorm(c.v[0]), float_to_unorm(c.v[1]), float_to_unorm(c.v[2]))
}

pub fn float_to_unorm_4(c: float4) -> byte4 {
    byte4::new(
        float_to_unorm(c.v[0]),
        float_to_unorm(c.v[1]),
        float_to_unorm(c.v[2]),
        float_to_unorm(c.v[3]),
    )
}
