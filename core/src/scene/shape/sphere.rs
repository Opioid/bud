use super::Intersection;
use super::Shape;
use base::math::{self, float3};
use scene::entity::ComposedTransformation;
use scene::Ray;

pub struct Sphere {}

fn intersect(
    hit_t: f32,
    ray: &Ray,
    transformation: &ComposedTransformation,
    intersection: &mut Intersection,
) {
    let p = ray.ray.point(hit_t);

    let n = (p - transformation.position).normalized();

    let xyz = transformation
        .rotation
        .transform_vector_transposed(&n)
        .normalized();

    let phi = -xyz.v[0].atan2(xyz.v[2]) + math::PI;
    let theta = xyz.v[1].acos();

    let sin_theta = theta.sin().max(0.00001);
    let (sin_phi, cos_phi) = phi.sin_cos();

    let t = float3::new(sin_theta * cos_phi, 0.0, sin_theta * sin_phi).normalized();

    intersection.p = p;

    intersection.geo_n = n;
    intersection.t = t;
    intersection.b = t.cross(&n);
    intersection.n = n;
}

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
                intersect(t0, ray, transformation, intersection);

                ray.ray.max_t = t0;
                return true;
            }

            let t1 = b + dist;

            if t1 > ray.ray.min_t && t1 < ray.ray.max_t {
                intersect(t1, ray, transformation, intersection);

                ray.ray.max_t = t1;
                return true;
            }
        }

        false
    }

    fn intersect_p(&self, ray: &Ray, transformation: &ComposedTransformation) -> bool {
        let v = transformation.position - ray.ray.org;
        let b = ray.ray.dir.dot(&v);

        let remedy_term = v - b * ray.ray.dir;

        let radius = transformation.scale.v[0];
        let discriminant = radius * radius - remedy_term.dot(&remedy_term);

        if discriminant > 0.0 {
            let dist = discriminant.sqrt();
            let t0 = b - dist;

            if t0 > ray.ray.min_t && t0 < ray.ray.max_t {
                return true;
            }

            let t1 = b + dist;

            if t1 > ray.ray.min_t && t1 < ray.ray.max_t {
                return true;
            }
        }

        false
    }
}
