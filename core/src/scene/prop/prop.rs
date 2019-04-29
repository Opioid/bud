use scene::entity::Entity;
use scene::material::Material;
use scene::shape::{Intersection, Shape};
use scene::Ray;

pub struct Prop<'a> {
    pub entity: Entity,
    shape: &'a dyn Shape,
    materials: Vec<&'a dyn Material>,
}

impl<'a> Prop<'a> {
    pub fn new(shape: &dyn Shape) -> Prop {
        Prop {
            entity: Entity::new(),
            shape,
            materials: Vec::new(),
        }
    }

    pub fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        return self
            .shape
            .intersect(ray, &self.entity.transformation_at(0), intersection);
    }

    pub unsafe fn material(&self, index: u32) -> &dyn Material {
        *self.materials.get_unchecked(index as usize)
    }
}
