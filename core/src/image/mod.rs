pub mod encoding;
pub mod typed_image;
pub mod writer;

use base::math::{float3, float4};

pub use self::writer::Writer;

pub type Float3 = self::typed_image::TypedImage<float3>;
pub type Float4 = self::typed_image::TypedImage<float4>;
