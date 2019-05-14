// use super::Identifiable;
use error::Error;
use resource::Manager;
use std::rc::Rc;

pub trait Provider<T: ?Sized> {
    fn load(&self, filename: &str, manager: &Manager) -> Result<Rc<T>, Error>;
}
