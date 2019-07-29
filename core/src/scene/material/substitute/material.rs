use scene::material;

pub struct Material {}

impl Material {
    pub fn new() -> Material {
        Material {}
    }
}

impl material::Material for Material {
}

