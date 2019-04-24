use super::Prop;
use scene::shape;

pub struct Intersection<'a, 'b> {
    pub prop: Option<&'a Prop<'b>>,

    pub geo: shape::Intersection,
}

impl<'a, 'b> Intersection<'a, 'b> {
    pub fn new() -> Intersection<'a, 'b> {
        Intersection {
            prop: None,
            geo: shape::Intersection::new(),
        }
    }
}
