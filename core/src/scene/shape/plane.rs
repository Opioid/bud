use super::Shape;
use scene::Ray;

pub struct Plane {}

impl Shape for Plane {
    fn intersect(&self, _ray: &mut Ray) -> bool {
        println! {"Plane::intersect()"};
        false
    }
}
