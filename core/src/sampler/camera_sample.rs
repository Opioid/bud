use base::math::{int2, float2};

pub struct CameraSample {
    pub pixel: int2,
    pub pixel_uv: float2,
}

impl CameraSample {
    pub fn new(pixel: int2) -> CameraSample {
        CameraSample {
            pixel,
            pixel_uv: float2::new(0.5, 0.5),
        }
    }
}
