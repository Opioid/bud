use super::Filter;
use base::math::InterpolatedFunction;

pub struct Gaussian {
    radius: f32,
    gaussian: InterpolatedFunction<f32>,
}

impl Gaussian {
    pub fn new(radius: f32, alpha: f32) -> Gaussian {
        let squared_radius = radius * radius;
        let exp = (-alpha * squared_radius).exp();

        let gaussian = |squared_d: f32| ((-alpha * squared_d).exp() - exp).max(0.0);

        Gaussian {
            radius,
            gaussian: InterpolatedFunction::from_fn(0.0, radius * radius, 256, gaussian),
        }
    }
}

impl Filter for Gaussian {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate_1(&self, d: f32) -> f32 {
        self.gaussian.call(d * d)
    }
}
