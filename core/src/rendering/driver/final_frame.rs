use super::driver::DriverBase;
use base::math::{float4, int2};
use base::random;
use exporting;
use rendering::integrator::surface::Integrator;
use sampler::CameraSample;
use scene::prop::Intersection;
use scene::{Ray, Scene};
use take::{Take, View};

pub struct FinalFrame {
    base: DriverBase,

    integrators: Vec<Box<dyn Integrator>>,
}

impl FinalFrame {
    pub fn new(take: &Take) -> FinalFrame {
        let dimensions = take.view.camera.sensor_dimensions();

        let mut ff = FinalFrame {
            base: DriverBase::new(dimensions),
            integrators: Vec::new(),
        };

        ff.integrators
            .push(take.surface_integrator_factory.create());

        ff
    }

    pub fn render(
        &mut self,
        scene: &Scene,
        view: &mut View,
        exporters: &mut [Box<dyn exporting::Sink>],
    ) {
        self.render_frame(scene, view);

        let sensor = view.camera.sensor_mut();

        sensor.resolve(&mut self.base.target);

        for e in exporters {
            e.write(&self.base.target);
        }
    }

    fn render_frame(&mut self, scene: &Scene, view: &mut View) {
        let camera = &mut view.camera;

        camera.update();

        let d = camera.sensor_dimensions();

        for y in 0..d.v[1] {
            for x in 0..d.v[0] {
                let sample = CameraSample::new(int2::new(x, y));

                let ray = camera.generate_ray(&sample);

                if let Some(mut ray) = ray {
                    let color = self.li(scene, &mut ray);

                    camera.sensor_mut().add_sample(&sample, &color);
                }
            }
        }
    }

    fn li(&mut self, scene: &Scene, ray: &mut Ray) -> float4 {
        let mut intersection = Intersection::new();

        let hit = self.base.worker.intersect(scene, ray, &mut intersection);

        if hit {
            return self.integrators[0].li(scene, ray, &mut intersection, &mut self.base.worker);
        }

        float4::identity()
    }
}
