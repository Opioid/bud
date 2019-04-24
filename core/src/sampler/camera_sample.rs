use base::math::int2;

pub struct CameraSample {
    pub pixel: int2,
}

impl CameraSample {
    pub fn new() -> CameraSample {
        CameraSample {
            pixel: int2::identity(),
        }
    }
}
