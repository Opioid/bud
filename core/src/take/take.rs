pub struct View {
    pub start_frame: u32,
    pub num_frames: u32,
}

impl View {
    pub fn new() -> View {
        View {
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
    pub fn new() -> Take {
        Take {
            scene_filename: String::new(),
            view: View::new(),
        }
    }
}
