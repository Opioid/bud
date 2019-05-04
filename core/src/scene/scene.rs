use super::material::Material;
use super::prop::{Intersection, Prop};
use super::shape::Shape;
use super::Ray;

pub struct Scene<'a> {
    finite_props: Vec<Box<Prop<'a>>>,
    infinite_props: Vec<Box<Prop<'a>>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            finite_props: Vec::new(),
            infinite_props: Vec::new(),
        }
    }

    pub fn create_prop(&mut self, shape: &'a dyn Shape) -> &mut Prop<'a> {
        let mut prop = Box::new(Prop::new(shape));

        if shape.is_finite() {
            self.finite_props.push(prop);
            self.finite_props.last_mut().unwrap()
        } else {
            self.infinite_props.push(prop);
            self.infinite_props.last_mut().unwrap()
        }
    }

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        let mut hit = false;

        for (i, p) in self.finite_props.iter().enumerate() {
            if p.intersect(ray, &mut intersection.geo) {
                intersection.prop = i as u32;
                hit = true;
            }
        }

        for (i, p) in self.infinite_props.iter().enumerate() {
            if p.intersect(ray, &mut intersection.geo) {
                intersection.prop = i as u32;
                hit = true;
            }
        }

        hit
    }

    pub fn visibility(&self, ray: &Ray) -> Option<f32> {
        for p in self.finite_props.iter() {
            if p.intersect_p(ray) {
                return None;
            }
        }

        for p in self.infinite_props.iter() {
            if p.intersect_p(ray) {
                return None;
            }
        }

        Some(1.0)
    }

    pub fn material(&self, prop: u32, part: u32) -> &dyn Material {
        unsafe {
            self.finite_props
                .get_unchecked(prop as usize)
                .material(part)
        }
    }
}
