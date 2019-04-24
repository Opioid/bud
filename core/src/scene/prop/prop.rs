use scene::entity::Entity;
use scene::shape::{Intersection, Shape};
use scene::Ray;

pub struct Prop<'a> {
    pub entity: Entity,
    shape: &'a dyn Shape,
}

impl<'a> Prop<'a> {
    pub fn new(shape: &dyn Shape) -> Prop {
        Prop {
            entity: Entity::new(),
            shape,
        }
    }

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        return self.shape.intersect(ray, intersection);
    }
}
