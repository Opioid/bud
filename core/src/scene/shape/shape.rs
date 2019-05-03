use super::Intersection;
use scene::entity::ComposedTransformation;
use scene::Ray;

pub trait Shape {
    fn intersect(
        &self,
        ray: &mut Ray,
        transformation: &ComposedTransformation,
        intersection: &mut Intersection,
    ) -> bool;

    fn intersect_p(&self, ray: &Ray, transformation: &ComposedTransformation) -> bool;
}
