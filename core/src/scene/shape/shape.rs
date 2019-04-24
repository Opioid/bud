use scene::Ray;

pub trait Shape {
    fn intersect(&self, ray: &mut Ray) -> bool;
}
