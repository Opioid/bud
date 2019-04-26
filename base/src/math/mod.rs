pub mod ray;
pub mod transformation;
pub mod vector2;
pub mod vector3;

pub use self::ray::Ray;
pub use self::transformation::Transformation;
pub use self::vector2::*;
pub use self::vector3::float3;

pub use std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}
