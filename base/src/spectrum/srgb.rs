use math::{float3, float4};

// convert sRGB linear value to sRGB gamma value
pub fn linear_to_gamma(c: f32) -> f32 {
    if c <= 0.0 {
        0.0
    } else if c < 0.0031308 {
        12.92 * c
    } else if c < 1.0 {
        1.055 * c.powf(0.41666) - 0.055
    } else {
        1.0
    }
}

pub fn linear_to_gamma_3(c: float3) -> float3 {
    float3::new(
        linear_to_gamma(c.v[0]),
        linear_to_gamma(c.v[1]),
        linear_to_gamma(c.v[2]),
    )
}

pub fn linear_to_gamma_4(c: float4) -> float4 {
    float4::new(
        linear_to_gamma(c.v[0]),
        linear_to_gamma(c.v[1]),
        linear_to_gamma(c.v[2]),
        c.v[3],
    )
}
