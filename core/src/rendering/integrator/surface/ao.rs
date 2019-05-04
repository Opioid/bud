use super::{Factory, Integrator};
use base::math::{self, float3, float4};
use rendering::Worker;
use sampler::{GoldenRatio, Random, Sampler};
use scene::prop::Intersection;
use scene::{Ray, Scene};

#[derive(Copy, Clone)]
struct Settings {
    num_samples: u32,
    radius: f32,
}

pub struct Ao {
    settings: Settings,

    sampler: GoldenRatio,
}

impl Ao {
    fn new(settings: Settings) -> Ao {
        Ao {
            settings,
            sampler: GoldenRatio::new(),
        }
    }
}

impl Integrator for Ao {
    fn prepare(&mut self, num_samples_per_pixel: u32) {
        self.sampler
            .resize(num_samples_per_pixel, self.settings.num_samples, 1, 1);
    }

    fn start_pixel(&mut self) {
        self.sampler.start_pixel();
    }

    fn li(
        &mut self,
        scene: &Scene,
        ray: &mut Ray,
        intersection: &mut Intersection,
        worker: &mut Worker,
    ) -> float4 {
        let num_samples_reciprocal = 1.0 / self.settings.num_samples as f32;

        let mut result = 0.0;

        let mut occlusion_ray = Ray::new(
            intersection.geo.offset_p(),
            float3::identity(),
            0.0,
            self.settings.radius,
            ray.time,
        );

        for _ in 0..self.settings.num_samples {
            let uv = self.sampler.generate_sample_2d(worker.rng(), 0);

            let hs = math::sample_hemisphere_cosine(uv);

            let ws = intersection.geo.tangent_to_world(hs);

            occlusion_ray.ray.set_direction(ws);

            if let Some(_) = worker.masked_visibility(scene, &occlusion_ray) {
                result += num_samples_reciprocal;
            }
        }

        return float4::new(result, result, result, 1.0);
    }
}

pub struct AoFactory {
    settings: Settings,
}

impl AoFactory {
    pub fn new(num_samples: u32, radius: f32) -> AoFactory {
        AoFactory {
            settings: Settings {
                num_samples,
                radius,
            },
        }
    }
}

impl Factory for AoFactory {
    fn create(&self) -> Box<dyn Integrator> {
        Box::new(Ao::new(self.settings))
    }
}
