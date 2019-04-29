use super::Intersection;
use super::Shape;
use scene::entity::ComposedTransformation;
use scene::Ray;

pub struct Plane {}

impl Shape for Plane {
    fn intersect(
        &self,
        ray: &mut Ray,
        transformation: &ComposedTransformation,
        intersection: &mut Intersection,
    ) -> bool {
        let normal = &transformation.rotation.r[2];

        let d = normal.dot(&transformation.position);
        let denom = -normal.dot(&ray.ray.dir);
        let numer = normal.dot(&ray.ray.org) - d;
        let hit_t = numer / denom;

        if hit_t > ray.ray.min_t && hit_t < ray.ray.max_t {
            let p = ray.ray.point(hit_t);

            intersection.p = p;

            ray.ray.max_t = hit_t;
            return true;
        }

        false
    }
}
