use super::driver::DriverBase;
use base::math::{float4, int2};
use base::random;
use base::thread;
use exporting;
use rendering::integrator::surface::Integrator;
use sampler::{CameraSample, Sampler};
use scene::prop::Intersection;
use scene::{Ray, Scene};
use take::{Take, View};

pub struct FinalFrame<'a> {
    base: DriverBase<'a>,

    integrators: Vec<Box<dyn Integrator>>,
    samplers: Vec<Box<dyn Sampler>>,
}

impl<'a> FinalFrame<'a> {
    pub fn new(thread_pool: &'a thread::Pool, take: &Take) -> FinalFrame<'a> {
        let dimensions = take.view.camera.sensor_dimensions();

        let mut ff = FinalFrame {
            base: DriverBase::new(thread_pool, dimensions),
            integrators: Vec::new(),
            samplers: Vec::new(),
        };

        let mut sif = take.surface_integrator_factory.create();
        sif.prepare(take.view.num_samples_per_pixel);
        ff.integrators.push(sif);

        let mut sf = take.sampler_factory.create();
        sf.resize(take.view.num_samples_per_pixel, 1, 2, 1);
        ff.samplers.push(sf);

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
                self.samplers[0].start_pixel();
                self.integrators[0].start_pixel();

                for s in 0..view.num_samples_per_pixel {
                    let sample = self.samplers[0].generate_camera_sample(
                        self.base.worker.rng(),
                        int2::new(x, y),
                        s,
                    );

                    let ray = camera.generate_ray(&sample);

                    if let Some(mut ray) = ray {
                        let color = self.li(scene, &mut ray);

                        camera.sensor_mut().add_sample(&sample, color);
                    }
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
