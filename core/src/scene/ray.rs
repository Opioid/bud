use base::math::float3;
use base::math::Ray as ray;

pub struct Ray {
    pub ray: ray,
    pub time: u64,
    pub wl: f32,
}

impl Ray {
    pub fn new(org: float3, dir: float3, min_t: f32, max_t: f32, time: u64) -> Ray {
        Ray { ray: ray::new(org, dir, min_t, max_t), time, wl: 0.0 }
    }
}
