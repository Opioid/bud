use base::math::Ray as ray;

pub struct Ray {
    pub ray: ray,
    pub time: u64,
    pub wl: f32,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            ray: ray::new(),
            time: 0,
            wl: 0.0,
        }
    }
}
