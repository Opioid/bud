use base::math::int2;
use base::thread;
use image::Float3;
use rendering::Worker;
use scene::Scene;
use take::View;

pub struct DriverBase<'a> {
    pub thread_pool: &'a thread::Pool,
    pub worker: Worker,
    pub target: Float3,
}

impl<'a> DriverBase<'a> {
    pub fn new(thread_pool: &'a thread::Pool, dimensions: int2) -> DriverBase {
        DriverBase {
            thread_pool,
            worker: Worker::new(),
            target: Float3::new(dimensions),
        }
    }
}
