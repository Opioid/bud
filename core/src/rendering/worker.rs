use base::random;
use scene::material::Material;
use scene::prop::Intersection;
use scene::{Ray, Scene};

pub struct Worker {
    rng: random::Generator,
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            rng: random::Generator::new(0, 0),
        }
    }

    pub fn intersect(&self, scene: &Scene, ray: &mut Ray, intersection: &mut Intersection) -> bool {
        scene.intersect(ray, intersection)
    }

    pub fn masked_visibility(&self, scene: &Scene, ray: &Ray) -> Option<f32> {
        scene.visibility(ray)
    }

    pub fn material<'a>(&self, scene: &'a Scene, prop: u32, part: u32) -> &'a dyn Material {
        scene.material(prop, part)
    }

    pub fn rng(&mut self) -> &mut random::Generator {
        &mut self.rng
    }
}
