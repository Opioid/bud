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
}

impl Take {
    pub fn new(camera: Box<dyn Camera>) -> Take {
        Take {
            scene_filename: String::new(),
            view: View::new(camera),
        }
    }
}
