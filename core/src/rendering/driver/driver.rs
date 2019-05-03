use base::math::int2;

use image::Float3;
use rendering::Worker;
use scene::Scene;
use take::View;

pub struct DriverBase {
    pub worker: Worker,
    pub target: Float3,
}

impl DriverBase {
    pub fn new(dimensions: int2) -> DriverBase {
        DriverBase {
            worker: Worker::new(),
            target: Float3::new(dimensions),
        }
    }
}
