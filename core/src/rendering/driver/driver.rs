use base::math::int2;

use image::Float3;
use take::View;

pub struct DriverBase<'a> {
    view: &'a View,
    pub target: Float3,
}

impl<'a> DriverBase<'a> {
    pub fn new(view: &'a View) -> DriverBase<'a> {
        DriverBase {
            view,
            target: Float3::new(view.camera.sensor_dimensions()),
        }
    }
}
