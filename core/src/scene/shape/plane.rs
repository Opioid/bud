use super::Intersection;
use super::Shape;
use scene::entity::ComposedTransformation;
use scene::Ray;

pub struct Plane {}

impl Shape for Plane {
    fn is_finite(&self) -> bool {
        false
    }

    fn intersect(
        &self,
        ray: &mut Ray,
        transformation: &ComposedTransformation,
        intersection: &mut Intersection,
    ) -> bool {
        let normal = transformation.rotation.r[2];

        let d = normal.dot(transformation.position);
        let denom = -normal.dot(ray.ray.dir);
        let numer = normal.dot(ray.ray.org) - d;
        let hit_t = numer / denom;

        if hit_t > ray.ray.min_t && hit_t < ray.ray.max_t {
            let p = ray.ray.point(hit_t);
            let t = -transformation.rotation.r[0];
            let b = -transformation.rotation.r[1];

            intersection.p = p;
            intersection.geo_n = normal;
            intersection.t = t;
            intersection.b = b;
            intersection.n = normal;
            intersection.uv.v[0] = t.dot(p) * transformation.scale.v[0];
            intersection.uv.v[1] = b.dot(p) * transformation.scale.v[1];

            intersection.part = 0;

            ray.ray.max_t = hit_t;
            return true;
        }

        false
    }

    fn intersect_p(&self, ray: &Ray, transformation: &ComposedTransformation) -> bool {
        let normal = &transformation.rotation.r[2];

        let d = normal.dot(transformation.position);
        let denom = -normal.dot(ray.ray.dir);
        let numer = normal.dot(ray.ray.org) - d;
        let hit_t = numer / denom;

        if hit_t > ray.ray.min_t && hit_t < ray.ray.max_t {
            return true;
        }

        false
    }
}
