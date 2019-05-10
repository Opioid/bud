use base::math::{float3, int3};

pub const RAY_MAX_T: f32 = 2000000.0;

pub const UNITS_PER_SECOND: u64 = 705600000;

const ORIGIN: f32 = 1.0 / 32.0;
const FLOAT_SCALE: f32 = 1.0 / 65536.0;
const INT_SCALE: f32 = 256.0;

#[inline]
fn int_as_float(x: i32) -> f32 {
    unsafe { std::mem::transmute::<i32, f32>(x) }
}

#[inline]
fn float_as_int(x: f32) -> i32 {
    unsafe { std::mem::transmute::<f32, i32>(x) }
}

#[inline]
pub fn offset_ray(p: float3, n: float3) -> float3 {
    let of_i = int3::from(INT_SCALE * n);

    let p_i = float3::new(
        int_as_float(float_as_int(p.v[0]) + if p.v[0] < 0.0 { -of_i.v[0] } else { of_i.v[0] }),
        int_as_float(float_as_int(p.v[1]) + if p.v[1] < 0.0 { -of_i.v[1] } else { of_i.v[1] }),
        int_as_float(float_as_int(p.v[2]) + if p.v[2] < 0.0 { -of_i.v[2] } else { of_i.v[2] }),
    );

    float3::new(
        if p.v[0].abs() < ORIGIN { p.v[0] + FLOAT_SCALE * n.v[0] } else { p_i.v[0] },
        if p.v[1].abs() < ORIGIN { p.v[1] + FLOAT_SCALE * n.v[1] } else { p_i.v[1] },
        if p.v[2].abs() < ORIGIN { p.v[2] + FLOAT_SCALE * n.v[2] } else { p_i.v[2] },
    )
}
