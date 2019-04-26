use std::ops;
use num::Zero;

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Zero + 'static> vec2<T> {
    pub fn identity() -> vec2<T> {
        vec2 { x: num::zero(), y: num::zero() }
    }

    pub fn new(x: T, y: T) -> vec2<T> {
        vec2 { x, y }
    }

    pub fn from<U: num::cast::AsPrimitive<T>>(other: vec2<U>) -> vec2<T> {
        vec2 {
            x: other.x.as_(),
            y: other.y.as_(),
        }
    }
}

impl<T: ops::Add> ops::Add<vec2<T>> for vec2<T> {
    type Output = vec2<T>;

    fn add(self, other: vec2<T>) -> vec2<T> {
        vec2{x: self.x + other.x, y: self.y + other.y, }
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

pub fn float2_from_int2(v: int2) -> float2 {
    float2::new(v.x as f32, v.y as f32)
}

