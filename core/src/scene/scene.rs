use super::prop::{Intersection, Prop};
use super::shape::Shape;
use super::Ray;

pub struct Scene<'a> {
    props: Vec<Box<Prop<'a>>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { props: Vec::new() }
    }

    pub fn create_prop(&mut self, shape: &'a dyn Shape) -> &mut Prop<'a> {
        self.props.push(Box::new(Prop::new(shape)));

        self.props.last_mut().unwrap()
    }

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection<'a>) -> bool {
        for p in self.props.iter() {
            p.intersect(ray, &mut intersection.geo);
        }

        intersection.prop = Some(&self.props[0]);

        true
    }
}
