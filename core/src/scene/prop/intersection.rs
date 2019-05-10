use rendering::Worker;
use scene::material::Material;
use scene::shape;
use scene::Scene;

pub struct Intersection {
    pub prop: u32,

    pub geo: shape::Intersection,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection { prop: 0xFFFFFFFF, geo: shape::Intersection::new() }
    }

    pub fn material<'a>(&self, scene: &'a Scene, worker: &'a Worker) -> &'a dyn Material {
        worker.material(scene, self.prop, self.geo.part)
    }
}
