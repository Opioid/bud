use super::vector2::vec2;
use super::vector3::vec3;
use num::Zero;
use std::ops;

#[derive(Copy, Clone, Debug, Default)]
#[allow(non_camel_case_types)]
pub struct vec4<T> {
    pub v: [T; 4],
}

impl<T> vec4<T>
where
    T: Copy + Zero + ops::Sub<Output = T> + ops::Mul<Output = T> + 'static,
{
    #[inline]
    pub fn identity() -> vec4<T> {
        vec4 {
            v: [num::zero(), num::zero(), num::zero(), num::zero()],
        }
    }

    #[inline]
    pub fn from_2_2(a: vec2<T>, b: vec2<T>) -> vec4<T> {
        vec4 {
            v: [a.v[0], a.v[1], b.v[0], b.v[1]],
        }
    }

    #[inline]
    pub fn from_3(v: vec3<T>, w: T) -> vec4<T> {
        vec4 {
            v: [v.v[0], v.v[1], v.v[2], w],
        }
    }

    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> vec4<T> {
        vec4 { v: [x, y, z, w] }
    }

    #[inline]
    pub fn xyz(self) -> vec3<T> {
        vec3 {
            v: [self.v[0], self.v[1], self.v[2]],
        }
    }
}

impl<T: Copy + ops::AddAssign> ops::AddAssign for vec4<T> {
    #[inline]
    fn add_assign(&mut self, other: vec4<T>) {
        self.v[0] += other.v[0];
        self.v[1] += other.v[1];
        self.v[2] += other.v[2];
        self.v[3] += other.v[3];
    }
}

impl ops::Mul<vec4<f32>> for f32 {
    type Output = vec4<f32>;

    #[inline]
    fn mul(self, v: vec4<f32>) -> vec4<f32> {
        vec4 {
            v: [self * v.v[0], self * v.v[1], self * v.v[2], self * v.v[3]],
        }
    }
}

impl ops::Div<f32> for vec4<f32> {
    type Output = vec4<f32>;

    #[inline]
    fn div(self, s: f32) -> vec4<f32> {
        vec4 {
            v: [self.v[0] / s, self.v[1] / s, self.v[2] / s, self.v[3] / s],
        }
    }
}

#[allow(non_camel_case_types)]
pub type byte4 = vec4<u8>;

#[allow(non_camel_case_types)]
pub type int4 = vec4<i32>;

#[allow(non_camel_case_types)]
pub type float4 = vec4<f32>;
