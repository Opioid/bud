use super::Intersection;
use super::Shape;
use scene::Ray;

pub struct Sphere {}

impl Shape for Sphere {
    fn intersect(&self, _ray: &mut Ray, intersection: &mut Intersection) -> bool {
        // println! {"Sphere::intersect()"};
        false
    }
}
