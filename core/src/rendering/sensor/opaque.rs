use super::sensor::{SensorBase, TypedSensor};
use base::math::{float4, int2};
use image::Float4;

pub struct Opaque {
    base: SensorBase,
    pixels: Vec<float4>,
}

impl TypedSensor for Opaque {
    fn new(dimensions: int2, exposure: f32) -> Self {
        Opaque {
            base: SensorBase::new(dimensions, exposure),
            pixels: vec![float4::identity(); (dimensions.v[0] * dimensions.v[1]) as usize],
        }
    }

    fn has_alpha_transparency(&self) -> bool {
        false
    }

    fn resolve(&self, target: &mut Float4) {
        let exposure_factor = self.base.exposure_factor;

        for (i, pixel) in self.pixels.iter().enumerate() {
            let color = pixel.xyz() / pixel.v[3];
            target.set_by_index(i as i32, float4::from_3(exposure_factor * color, 1.0));
        }
    }

    fn add_pixel(&mut self, pixel: int2, color: float4, weight: f32) {
        let i = pixel.v[1] * self.base.dimensions.v[0] + pixel.v[0];

        let value = float4::from_3(weight * color.xyz(), weight);

        unsafe {
            *self.pixels.get_unchecked_mut(i as usize) += value;
        }
    }
}
