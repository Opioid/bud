use scene::entity::Entity;
use scene::material::Material;
use scene::shape::{Intersection, Shape};
use scene::Ray;
use std::rc::Rc;

pub struct Prop<'a> {
    pub entity: Entity,
    shape: &'a dyn Shape,
    materials: Vec<Rc<dyn Material>>,
}

impl<'a> Prop<'a> {
    pub fn new(shape: &dyn Shape, materials: Vec<Rc<dyn Material>>) -> Prop {
        Prop { entity: Entity::new(), shape, materials }
    }

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        return self.shape.intersect(ray, &self.entity.transformation_at(0), intersection);
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        return self.shape.intersect_p(ray, &self.entity.transformation_at(0));
    }

    pub unsafe fn material(&self, index: u32) -> &dyn Material {
        self.materials.get_unchecked(index as usize).as_ref()
    }
}
