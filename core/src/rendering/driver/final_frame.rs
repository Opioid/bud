use super::driver::DriverBase;
use base::math::{float3, int2};
use exporting;
use sampler::CameraSample;
use scene::prop::Intersection;
use scene::Scene;
use take::View;

pub struct FinalFrame<'a> {
    base: DriverBase<'a>,
}

impl<'a> FinalFrame<'a> {
    pub fn new(view: &'a mut View, scene: &'a Scene) -> FinalFrame<'a> {
        FinalFrame {
            base: DriverBase::new(view, scene),
        }
    }

    pub fn render(&mut self, exporters: &mut [Box<dyn exporting::Sink>]) {
        self.render_frame();

        let sensor = self.base.view.camera.sensor_mut();

        sensor.resolve(&mut self.base.target);

        for e in exporters {
            e.write(&self.base.target);
        }
    }

    fn render_frame(&mut self) {
        let camera = &mut (*self.base.view.camera);

        camera.update();

        let d = camera.sensor_dimensions();

        for y in 0..d.v[1] {
            for x in 0..d.v[0] {
                let sample = CameraSample::new(int2::new(x, y));

                let ray = camera.generate_ray(&sample);

                if let Some(mut ray) = ray {
                    // println!("{:?}", ray.ray.dir);

                    let mut intersection = Intersection::new();

                    self.base.scene.intersect(&mut ray, &mut intersection);
                }

                let color = float3::new(1.0, 0.0, 0.0);

                camera.sensor_mut().add_sample(&sample, &color);
            }
        }
    }
}
