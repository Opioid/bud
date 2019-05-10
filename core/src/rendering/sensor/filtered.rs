use super::filter::Filter;
use super::sensor::{Sensor, TypedSensor};
use base::math::{float4, int2, int4};
use image::Float4;
use sampler::CameraSample;

pub struct Filtered1p0<T, F> {
    base: T,
    filter: F,
}

impl<T, F> Filtered1p0<T, F>
where
    T: TypedSensor,
    F: Filter,
{
    pub fn new(dimensions: int2, exposure: f32, filter: F) -> Filtered1p0<T, F> {
        Filtered1p0 { base: T::new(dimensions, exposure), filter }
    }
}

impl<T, F> Sensor for Filtered1p0<T, F>
where
    T: TypedSensor,
    F: Filter,
{
    fn has_alpha_transparency(&self) -> bool {
        self.base.has_alpha_transparency()
    }

    fn resolve(&self, target: &mut Float4) {
        self.base.resolve(target)
    }

    fn filter_radius_int(&self) -> i32 {
        self.filter.radius().ceil() as i32
    }

    fn add_sample(&mut self, sample: &CameraSample, color: float4, bounds: int4) {
        let clamped = color;

        let x = bounds.v[0] + sample.pixel.v[0];
        let y = bounds.v[1] + sample.pixel.v[1];

        let ox = sample.pixel_uv.v[0] - 0.5;
        let oy = sample.pixel_uv.v[1] - 0.5;

        let wx0 = self.filter.evaluate_1(ox + 1.0);
        let wx1 = self.filter.evaluate_1(ox);
        let wx2 = self.filter.evaluate_1(ox - 1.0);

        let wy0 = self.filter.evaluate_1(oy + 1.0);
        let wy1 = self.filter.evaluate_1(oy);
        let wy2 = self.filter.evaluate_1(oy - 1.0);

        // 1. row
        add_weighted(&mut self.base, int2::new(x - 1, y - 1), wx0 * wy0, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x, y - 1), wx1 * wy0, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x + 1, y - 1), wx2 * wy0, clamped, bounds);

        // 2. row
        add_weighted(&mut self.base, int2::new(x - 1, y), wx0 * wy1, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x, y), wx1 * wy1, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x + 1, y), wx2 * wy1, clamped, bounds);

        // 3. row
        add_weighted(&mut self.base, int2::new(x - 1, y + 1), wx0 * wy2, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x, y + 1), wx1 * wy2, clamped, bounds);
        add_weighted(&mut self.base, int2::new(x + 1, y + 1), wx2 * wy2, clamped, bounds);
    }
}

fn add_weighted<T: TypedSensor>(
    sensor: &mut T,
    pixel: int2,
    weight: f32,
    color: float4,
    bounds: int4,
) {
    if (pixel.v[0] - bounds.v[0]) as u32 <= bounds.v[2] as u32
        && (pixel.v[1] - bounds.v[1]) as u32 <= bounds.v[3] as u32
    {
        sensor.add_pixel(pixel, color, weight)
    }
}
