use base::math::float3;
use scene;

pub struct Intersection {
    // position in world space or texture space
    pub p: float3,

    // geometry normal in world space
    pub geo_n: float3,

    // interpolated tangent frame in world space
    pub t: float3,
    pub b: float3,
    pub n: float3,

    pub part: u32,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            p: float3::identity(),
            geo_n: float3::identity(),
            t: float3::identity(),
            b: float3::identity(),
            n: float3::identity(),
            part: 0,
        }
    }

    #[inline]
    pub fn tangent_to_world(&self, v: float3) -> float3 {
        float3::new(
            v.v[0] * self.t.v[0] + v.v[1] * self.b.v[0] + v.v[2] * self.n.v[0],
            v.v[0] * self.t.v[1] + v.v[1] * self.b.v[1] + v.v[2] * self.n.v[1],
            v.v[0] * self.t.v[2] + v.v[1] * self.b.v[2] + v.v[2] * self.n.v[2],
        )
    }

    #[inline]
    pub fn offset_p(&self) -> float3 {
        scene::offset_ray(self.p, self.geo_n)
    }
}
