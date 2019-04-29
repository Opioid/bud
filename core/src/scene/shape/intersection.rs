use base::math::float3;

pub struct Intersection {
    pub p: float3,

    pub geo_n: float3,

    pub part: u32,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            p: float3::identity(),
            geo_n: float3::identity(),
            part: 0,
        }
    }
}
