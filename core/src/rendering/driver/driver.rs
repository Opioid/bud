use base::math::int2;

use image::Float3;
use rendering::Worker;
use scene::Scene;
use take::View;

pub struct DriverBase<'a> {
    pub scene: &'a Scene<'a>,
    pub worker: Worker<'a>,
    pub target: Float3,
}

impl<'a> DriverBase<'a> {
    pub fn new(dimensions: int2, scene: &'a Scene) -> DriverBase<'a> {
        DriverBase {
            scene,
            worker: Worker::new(scene),
            target: Float3::new(dimensions),
        }
    }
}
