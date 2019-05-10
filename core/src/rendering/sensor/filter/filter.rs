pub trait Filter {
    fn radius(&self) -> f32;

    fn evaluate_1(&self, d: f32) -> f32;
}
