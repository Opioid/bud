use super::driver::DriverBase;
use exporting;
use take::View;
use sampler::CameraSample;
use scene::Scene;
use scene::prop::Intersection;
use base::math::{int2, float3};

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

        for y in 0..d.y {
            for x in 0..d.x {
                let sample = CameraSample::new(int2::new(x, y));

                let ray = camera.generate_ray(&sample);

                if let Some(mut ray) = ray {
                    let mut intersection = Intersection::new();

                    self.base.scene.intersect(&mut ray, &mut intersection);
                }

                let color = float3::new(1.0, 0.0, 0.0);

                camera.sensor_mut().add_sample(&sample, &color);
            }
        }

        
    }
}
