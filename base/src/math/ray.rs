use super::float3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub org: float3,
    pub dir: float3,

    pub min_t: f32,
    pub max_t: f32,
}

impl Ray {
    pub fn new(org: float3, dir: float3, min_t: f32, max_t: f32) -> Ray {
        Ray {
            org,
            dir,
            min_t,
            max_t,
        }
    }

    pub fn point(&self, t: f32) -> float3 {
        return self.org + t * self.dir;
    }
}
