use super::Intersection;
use super::Shape;
use scene::entity::ComposedTransformation;
use scene::Ray;

pub struct Sphere {}

impl Shape for Sphere {
    fn intersect(
        &self,
        ray: &mut Ray,
        transformation: &ComposedTransformation,
        intersection: &mut Intersection,
    ) -> bool {
        let v = transformation.position - ray.ray.org;
        let b = ray.ray.dir.dot(&v);

        let remedy_term = v - b * ray.ray.dir;

        let radius = transformation.scale.v[0];
        let discriminant = radius * radius - remedy_term.dot(&remedy_term);

        if discriminant > 0.0 {
            let dist = discriminant.sqrt();
            let t0 = b - dist;

            if t0 > ray.ray.min_t && t0 < ray.ray.max_t {
                ray.ray.max_t = t0;
                return true;
            }

            let t1 = b + dist;

            if t1 > ray.ray.min_t && t1 < ray.ray.max_t {
                ray.ray.max_t = t1;
                return true;
            }
        }

        false
    }
}
