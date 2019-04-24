use super::Prop;
use scene::shape;

pub struct Intersection<'a> {
    pub prop: Option<&'a Prop<'a>>,

    pub geo: shape::Intersection,
}

impl<'a> Intersection<'a> {
    pub fn new() -> Intersection<'a> {
        Intersection {
            prop: None,
            geo: shape::Intersection::new(),
        }
    }
}
