use super::driver::DriverBase;
use base::math::{float3, float4, int2};
use exporting;
use rendering::integrator::surface::{AoFactory, Integrator};
use sampler::CameraSample;
use scene::prop::Intersection;
use scene::{Ray, Scene};
use take::View;

pub struct FinalFrame<'a, 'b> {
    base: DriverBase<'a>,
    integrator: Box<dyn Integrator<'a, 'b>>,
}

impl<'a, 'b> FinalFrame<'a, 'b> {
    pub fn new(dimensions: int2, scene: &'a Scene) -> FinalFrame<'a, 'b> {
        FinalFrame {
            base: DriverBase::new(dimensions, scene),
            integrator: AoFactory::create(),
        }
    }

    pub fn render(&'b mut self, view: &mut View, exporters: &mut [Box<dyn exporting::Sink>]) {
        self.render_frame(view);

        let sensor = view.camera.sensor_mut();

        sensor.resolve(&mut self.base.target);

        for e in exporters {
            e.write(&self.base.target);
        }
    }

    fn render_frame(&'b mut self, view: &mut View) {
        let camera = &mut view.camera;

        camera.update();

        let d = camera.sensor_dimensions();

        for y in 0..d.v[1] {
            for x in 0..d.v[0] {
                let sample = CameraSample::new(int2::new(x, y));

                let ray = camera.generate_ray(&sample);

                if let Some(mut ray) = ray {
                    let color = self.li(&mut ray);

                    camera.sensor_mut().add_sample(&sample, &color);
                }
            }
        }
    }

    fn li(&'b mut self, ray: &mut Ray) -> float4 {
        let mut intersection = Intersection::new();

        let hit = self.base.worker.intersect(ray, &mut intersection);

        if hit {
            return self
                .integrator
                .li(ray, &mut intersection, &mut self.base.worker);
        }

        float4::identity()
    }
}
