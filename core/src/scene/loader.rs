use error::Error;
use resource;
use scene::Scene;
use serde_json::Value;

pub struct Loader {
    resource_manager: resource::Manager,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            resource_manager: resource::Manager::new(),
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<Scene, Error> {
        let mut stream = self.resource_manager.file_system().read_stream(filename)?;

        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new("No suitable root object."));
        }

        let root = root.as_object().unwrap();

        let mut scene = Scene::new();

        Ok(scene)
    }

    pub fn resource_manager(&mut self) -> &mut resource::Manager {
        &mut self.resource_manager
    }
}
