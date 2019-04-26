use num::Zero;

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq + Zero> vec2<T> {
    pub fn identity() -> vec2<T> {
        vec2 { x: num::zero(), y: num::zero() }
    }

    pub fn new(x: T, y: T) -> vec2<T> {
        vec2 { x, y }
    }
}

// impl<T: PartialEq> PartialEq for vec2<T> {
//     fn eq(&self, other: &vec2<T>) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

#[allow(non_camel_case_types)]
pub type int2 = vec2<i32>;

#[allow(non_camel_case_types)]
pub type float2 = vec2<f32>;
