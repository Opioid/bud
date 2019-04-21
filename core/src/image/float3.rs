use crate::base::math::vector2::int2;
use crate::base::math::vector3::float3;

pub struct Float3 {
    pub dimensions: int2,
    data: Vec<float3>,
}

impl Float3 {
    pub fn new(dimensions: int2) -> Float3 {
        Float3 {
            dimensions,
            data: vec![float3::from_scalar(0.0); (dimensions.x * dimensions.y) as usize],
        }
    }

    pub fn get_by_index(&self, i: i32) -> float3 {
        self.data[i as usize]
    }

    pub fn set_by_index(&mut self, i: i32, v: float3) {
        self.data[i as usize] = v;
    }

    pub fn set(&mut self, x: i32, y: i32, v: float3) {
        let i = y * self.dimensions.x + x;
        self.data[i as usize] = v;
    }
}
