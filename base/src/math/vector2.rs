use num::Zero;
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct vec2<T> {
    pub v: [T; 2],
}

impl<T: Copy + Zero + 'static> vec2<T> {
    pub fn identity() -> vec2<T> {
        vec2 {
            v: [num::zero(), num::zero()],
        }
    }

    pub fn new(x: T, y: T) -> vec2<T> {
        vec2 { v: [x, y] }
    }

    pub fn from<U: num::cast::AsPrimitive<T>>(other: vec2<U>) -> vec2<T> {
        vec2 {
            v: [other.v[0].as_(), other.v[1].as_()],
        }
    }
}

impl<T: Copy + ops::Add<Output = T>> ops::Add<vec2<T>> for vec2<T> {
    type Output = vec2<T>;

    fn add(self, other: vec2<T>) -> vec2<T> {
        vec2 {
            v: [self.v[0] + other.v[0], self.v[1] + other.v[1]],
        }
    }
}

impl ops::Sub<f32> for vec2<f32> {
    type Output = vec2<f32>;

    fn sub(self, s: f32) -> vec2<f32> {
        vec2 {
            v: [self.v[0] - s, self.v[1] - s],
        }
    }
}

impl ops::Mul<vec2<f32>> for f32 {
    type Output = vec2<f32>;

    fn mul(self, v: vec2<f32>) -> vec2<f32> {
        vec2 {
            v: [self * v.v[0], self * v.v[1]],
        }
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
