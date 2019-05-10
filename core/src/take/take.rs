use exporting;
use rendering::integrator::surface::Factory as SurfaceFactory;
use sampler::Factory as SamplerFactory;
use scene::camera::Camera;

pub struct View {
    pub camera: Box<dyn Camera>,

    pub num_samples_per_pixel: u32,

    pub start_frame: u32,
    pub num_frames: u32,
}

impl View {
    pub fn new(camera: Box<dyn Camera>) -> View {
        View { camera, num_samples_per_pixel: 1, start_frame: 0, num_frames: 1 }
    }
}

pub struct Take {
    pub scene_filename: String,
    pub view: View,
    pub surface_integrator_factory: Box<dyn SurfaceFactory>,
    pub sampler_factory: Box<dyn SamplerFactory>,
    pub exporters: Vec<Box<dyn exporting::Sink>>,
}

impl Take {
    pub fn new(
        camera: Box<dyn Camera>,
        surface_integrator_factory: Box<dyn SurfaceFactory>,
        sampler_factory: Box<dyn SamplerFactory>,
    ) -> Take {
        Take {
            scene_filename: String::new(),
            view: View::new(camera),
            surface_integrator_factory,
            sampler_factory,
            exporters: Vec::new(),
        }
    }
}
