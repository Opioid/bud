use super::vector2::vec2;
use num::Zero;
use std::ops;

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct vec3<T> {
    pub v: [T; 3],
}

impl<T: Copy + Zero + ops::Sub<Output = T> + ops::Mul<Output = T> + 'static> vec3<T> {
    #[inline]
    pub fn identity() -> vec3<T> {
        vec3 {
            v: [num::zero(), num::zero(), num::zero()],
        }
    }

    #[inline]
    pub fn from_scalar(s: T) -> vec3<T> {
        vec3 { v: [s, s, s] }
    }

    #[inline]
    pub fn from_2(v: vec2<T>, z: T) -> vec3<T> {
        vec3 {
            v: [v.v[0], v.v[1], z],
        }
    }

    #[inline]
    pub fn new(x: T, y: T, z: T) -> vec3<T> {
        vec3 { v: [x, y, z] }
    }

    #[inline]
    pub fn from<U: num::cast::AsPrimitive<T>>(other: vec3<U>) -> vec3<T> {
        vec3 {
            v: [other.v[0].as_(), other.v[1].as_(), other.v[2].as_()],
        }
    }

    #[inline]
    pub fn dot(self, other: vec3<T>) -> T {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    // pub fn length(&self) -> T {
    //     self.dot(self).sqrt()
    // }

    // pub fn normalized(&self) -> vec3<T> {
    //     *self / self.length()
    // }

    #[inline]
    pub fn cross(self, b: vec3<T>) -> vec3<T> {
        vec3 {
            v: [
                self.v[1] * b.v[2] - self.v[2] * b.v[1],
                self.v[2] * b.v[0] - self.v[0] * b.v[2],
                self.v[0] * b.v[1] - self.v[1] * b.v[0],
            ],
        }
    }
}

impl vec3<f32> {
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn normalized(self) -> vec3<f32> {
        self / self.length()
    }
}

impl ops::Add<float3> for float3 {
    type Output = float3;

    #[inline]
    fn add(self, other: float3) -> float3 {
        float3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
            ],
        }
    }
}

impl ops::Sub<float3> for float3 {
    type Output = float3;

    #[inline]
    fn sub(self, other: float3) -> float3 {
        float3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

impl ops::Neg for float3 {
    type Output = float3;

    #[inline]
    fn neg(self) -> float3 {
        float3 {
            v: [-self.v[0], -self.v[1], -self.v[2]],
        }
    }
}

impl ops::Mul<vec3<f32>> for f32 {
    type Output = vec3<f32>;

    #[inline]
    fn mul(self, v: vec3<f32>) -> vec3<f32> {
        vec3 {
            v: [self * v.v[0], self * v.v[1], self * v.v[2]],
        }
    }
}

impl ops::Div<f32> for float3 {
    type Output = float3;

    #[inline]
    fn div(self, s: f32) -> float3 {
        float3 {
            v: [self.v[0] / s, self.v[1] / s, self.v[2] / s],
        }
    }
}

#[allow(non_camel_case_types)]
pub type int3 = vec3<i32>;

#[allow(non_camel_case_types)]
pub type float3 = vec3<f32>;
