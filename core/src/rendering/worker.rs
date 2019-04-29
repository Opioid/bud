use scene::material::Material;
use scene::prop::Intersection;
use scene::{Ray, Scene};

pub struct Worker<'a> {
    scene: &'a Scene<'a>,
}

impl<'a, 'b> Worker<'a> {
    pub fn new(scene: &'a Scene) -> Worker<'a> {
        Worker { scene }
    }

    pub fn intersect(&'b self, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        self.scene.intersect(ray, intersection)
    }

    pub fn masked_visibility(&'b self, ray: &Ray) -> Option<f32> {
        self.scene.visibility(ray)
    }

    pub fn material(&self, prop: u32, part: u32) -> &'a dyn Material {
        self.scene.material(prop, part)
    }
}
