use base::math::int2;
use base::thread;
use image::Float4;
use rendering::TileQueue;
use rendering::Worker;
use scene::camera::Camera;
use scene::Scene;
use take::View;

pub struct DriverBase<'a> {
    pub thread_pool: &'a thread::Pool,
    pub worker: Worker,
    pub tiles: TileQueue,
    pub target: Float4,
}

impl<'a> DriverBase<'a> {
    pub fn new(thread_pool: &'a thread::Pool, camera: &dyn Camera) -> DriverBase<'a> {
        DriverBase {
            thread_pool,
            worker: Worker::new(),
            tiles: TileQueue::new(camera.resolution(), int2::new(32, 32), 0),
            target: Float4::new(camera.sensor_dimensions()),
        }
    }
}
