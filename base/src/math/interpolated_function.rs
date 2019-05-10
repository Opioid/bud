use super::Lerp;

pub struct InterpolatedFunction<T> {
    range_end: f32,
    inverse_range: f32,
    samples: Vec<T>,
}

impl<T> InterpolatedFunction<T>
where
    T: Lerp<T> + Copy + Clone,
{
    pub fn from_fn<F: (Fn(f32) -> T)>(
        range_begin: f32,
        range_end: f32,
        num_samples: usize,
        f: F,
    ) -> InterpolatedFunction<T> {
        let range = range_end - range_begin;

        let interval = range / (num_samples - 1) as f32;

        let inverse_range = 1.0 / interval;

        let mut samples = Vec::with_capacity(num_samples + 1);

        let mut s = range_begin;
        for _ in 0..num_samples {
            samples.push(f(s));
            s += interval;
        }

        samples.push(f(range_end));

        InterpolatedFunction { range_end, inverse_range, samples }
    }

    pub fn call(&self, x: f32) -> T {
        let x = x.min(self.range_end);

        let o = x * self.inverse_range;

        let offset = o as u32;

        let t = o - offset as f32;

        self.samples[offset as usize].lerp(self.samples[(offset + 1) as usize], t)
    }
}
