use super::Material;
use error::Error;
use resource::{self, Identifiable, Manager};
use std::rc::Rc;

pub struct Provider {}

impl resource::Provider<dyn Material> for Provider {
    fn load(&self, filename: &str, manager: &Manager) -> Result<Rc<dyn Material>, Error> {
        let stream = manager.file_system().read_stream(filename)?;

        Err(Error::new("No material implemented yet"))
    }
}
