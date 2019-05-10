use super::sensor::{SensorBase, TypedSensor};
use base::math::{float4, int2};
use image::Float4;

#[derive(Copy, Clone)]
struct Pixel {
    color: float4,
    weight_sum: f32,
}

pub struct Transparent {
    base: SensorBase,
    pixels: Vec<Pixel>,
}

impl TypedSensor for Transparent {
    fn new(dimensions: int2, exposure: f32) -> Self {
        Transparent {
            base: SensorBase::new(dimensions, exposure),
            pixels: vec![
                Pixel { color: float4::identity(), weight_sum: 0.0 };
                (dimensions.v[0] * dimensions.v[1]) as usize
            ],
        }
    }

    fn has_alpha_transparency(&self) -> bool {
        true
    }

    fn resolve(&self, target: &mut Float4) {
        let exposure_factor = self.base.exposure_factor;

        for (i, pixel) in self.pixels.iter().enumerate() {
            let color = pixel.color / pixel.weight_sum;
            target
                .set_by_index(i as i32, float4::from_3(exposure_factor * color.xyz(), color.v[3]));
        }
    }

    fn add_pixel(&mut self, pixel: int2, color: float4, weight: f32) {
        let i = pixel.v[1] * self.base.dimensions.v[0] + pixel.v[0];

        unsafe {
            let value = self.pixels.get_unchecked_mut(i as usize);
            value.color += weight * color;
            value.weight_sum += weight;
        }
    }
}
