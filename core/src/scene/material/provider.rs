use super::Material;
use resource;
use std::rc::Rc;

pub struct Provider {}

struct Matte {}

impl Material for Matte {}

impl resource::Provider<dyn Material> for Provider {
    fn load(&self, name: &str) -> Rc<dyn Material> {
        println!("We load materials");
        Rc::new(Matte {})
    }
}
