pub mod interpolated_function;
pub mod matrix3x3;
pub mod matrix4x4;
pub mod quaternion;
pub mod ray;
pub mod sample_distribution;
pub mod sampling;
pub mod transformation;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub use self::interpolated_function::InterpolatedFunction;
pub use self::matrix3x3::float3x3;
pub use self::matrix4x4::float4x4;
pub use self::quaternion::Quaternion;
pub use self::ray::Ray;
pub use self::sample_distribution::*;
pub use self::sampling::*;
pub use self::transformation::Transformation;
pub use self::vector2::*;
pub use self::vector3::*;
pub use self::vector4::*;

pub use std::f32::consts::PI;

pub const PI_INV: f32 = 1.0 / PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

pub trait Lerp<T> {
    fn lerp(self, other: T, t: f32) -> T;
}

impl Lerp<f32> for f32 {
    fn lerp(self, other: f32, t: f32) -> f32 {
        let u = 1.0 - t;
        u * self + t * other
    }
}
