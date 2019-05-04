use base::math::{float2, int2};

pub struct CameraSample {
    pub pixel: int2,
    pub pixel_uv: float2,
    pub time: f32,
}

impl CameraSample {
    pub fn new(pixel: int2, pixel_uv: float2) -> CameraSample {
        CameraSample {
            pixel,
            pixel_uv,
            time: 0.0,
        }
    }
}
