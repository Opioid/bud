use super::Material;
use resource::{self, Identifiable, Manager};
use scene::material::light;
use scene::material::substitute;
use base::math::float3;
use error::Error;
use serde_json::{Map, Value};
use std::rc::Rc;

pub struct Provider {}

impl resource::Provider<dyn Material> for Provider {
    fn load(&self, filename: &str, manager: &Manager) -> Result<Rc<dyn Material>, Error> {
        let stream = manager.file_system().read_stream(filename)?;

        let root: Value = serde_json::from_reader(stream)?;

        load(&root, manager)
    }

}

fn load(value: &Value, manager: &Manager) -> Result<Rc<dyn Material>, Error> {
    let rendering_node = match value.get("rendering") {
        Some(Value::Object(rendering_node)) => rendering_node,
        _ => return Err(Error::new("Material has no render node.")),
    };

        for (name, value) in rendering_node.iter() {
            match name.as_ref() {
                "Light" => return Ok(load_light(value, manager)),
                "Substitute" => return Ok(load_substitute(value, manager)),
                _ => (),
            }
        }

    Err(Error::new("Material is of unknown type."))
}

fn load_light(substitute_value: &Value, manager: &Manager) -> Rc<dyn Material> {
    println!("Light");

    let mut material = light::Constant::new();

    *material.emittance() = float3::new(1.0, 0.5, 0.25);

    Rc::new(material)
}

fn load_substitute(substitute_value: &Value, manager: &Manager) -> Rc<dyn Material> {
    println!("Substitute");

    Rc::new(substitute::Material::new())
}
