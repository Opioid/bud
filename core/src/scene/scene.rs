use super::material::Material;
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

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        let mut hit = false;

        for (i, p) in self.props.iter().enumerate() {
            if p.intersect(ray, &mut intersection.geo) {
                intersection.prop = i as u32;
                hit = true;
            }
        }

        hit
    }

    pub fn visibility(&self, ray: &Ray) -> Option<f32> {
        for p in self.props.iter() {
            if p.intersect_p(ray) {
                return None;
            }
        }

        Some(1.0)
    }

    pub fn material(&self, prop: u32, part: u32) -> &dyn Material {
        unsafe { self.props.get_unchecked(prop as usize).material(part) }
    }
}
