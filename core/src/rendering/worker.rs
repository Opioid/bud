use scene::material::Material;
use scene::prop::Intersection;
use scene::{Ray, Scene};
use base::random;

pub struct Worker<'a> {
    rng: random::Generator,
    scene: &'a Scene<'a>,
}

impl<'a, 'b> Worker<'a> {
    pub fn new(scene: &'a Scene) -> Worker<'a> {
        Worker { rng: random::Generator::new(0, 0), scene }
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

    pub fn rng(&mut self) -> &mut random::Generator {
        &mut self.rng
    }
}
