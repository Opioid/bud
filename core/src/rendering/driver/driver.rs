use base::math::int2;

use image::Float3;
use scene::Scene;
use take::View;

pub struct DriverBase<'a> {
    pub view: &'a mut View,
    pub scene: &'a Scene<'a>,
    pub target: Float3,
}

impl<'a> DriverBase<'a> {
    pub fn new(view: &'a mut View, scene: &'a Scene) -> DriverBase<'a> {
        let d = view.camera.sensor_dimensions();
        DriverBase {
            view,
            scene,
            target: Float3::new(d),
        }
    }
}
