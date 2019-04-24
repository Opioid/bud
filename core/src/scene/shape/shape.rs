use super::Intersection;
use scene::Ray;

pub trait Shape {
    fn intersect(&self, ray: &mut Ray, intersection: &mut Intersection) -> bool;
}
