use rendering::Worker;
use scene::material::Material;
use scene::shape;

pub struct Intersection {
    pub prop: u32,

    pub geo: shape::Intersection,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            prop: 0xFFFFFFFF,
            geo: shape::Intersection::new(),
        }
    }

    pub fn material<'a>(&self, worker: &'a Worker) -> &'a dyn Material {
        worker.material(self.prop, self.geo.part)
    }
}
