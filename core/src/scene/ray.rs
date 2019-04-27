use base::math::float3;
use base::math::Ray as ray;

pub struct Ray {
    pub ray: ray,
    pub time: u64,
    pub wl: f32,
}

impl Ray {
    pub fn new(org: float3, dir: float3) -> Ray {
        Ray {
            ray: ray::new(org, dir),
            time: 0,
            wl: 0.0,
        }
    }
}
