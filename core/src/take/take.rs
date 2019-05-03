use exporting;
use rendering::integrator::surface::Factory;
use scene::camera::Camera;

pub struct View {
    pub camera: Box<dyn Camera>,

    pub start_frame: u32,
    pub num_frames: u32,
}

impl View {
    pub fn new(camera: Box<dyn Camera>) -> View {
        View {
            camera,
            start_frame: 0,
            num_frames: 1,
        }
    }
}

pub struct Take {
    pub scene_filename: String,
    pub view: View,
    pub surface_integrator_factory: Box<dyn Factory>,
    pub exporters: Vec<Box<dyn exporting::Sink>>,
}

impl Take {
    pub fn new(camera: Box<dyn Camera>, surface_integrator_factory: Box<dyn Factory>) -> Take {
        Take {
            scene_filename: String::new(),
            view: View::new(camera),
            surface_integrator_factory,
            exporters: Vec::new(),
        }
    }
}
