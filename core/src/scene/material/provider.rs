use super::Material;
use error::Error;
use resource::{self, Identifiable, Manager};
use serde_json::{Map, Value};
use std::rc::Rc;

pub struct Provider {}

impl resource::Provider<dyn Material> for Provider {
    fn load(&self, filename: &str, manager: &Manager) -> Result<Rc<dyn Material>, Error> {
        let stream = manager.file_system().read_stream(filename)?;

        let root: Value = serde_json::from_reader(stream)?;
        let root = match root {
            Value::Object(root) => root,
            _ => return Err(Error::new("No suitable root object.")),
        };

        for (name, value) in root.iter() {
            match name.as_ref() {
                "Something" => (),
                _ => println!("{}", name),
            }
        }

        Err(Error::new("No material implemented yet"))
    }
}
