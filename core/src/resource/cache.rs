use error::Error;
use resource::{self, Manager};
use std::collections::HashMap;
use std::rc::Rc;

pub trait Cache {}

pub struct TypedCache<T: ?Sized> {
    provider: Box<resource::Provider<T>>,
    resources: HashMap<String, Rc<T>>,
}

impl<T: ?Sized> TypedCache<T> {
    pub fn new(provider: Box<resource::Provider<T>>) -> TypedCache<T> {
        TypedCache { provider, resources: HashMap::new() }
    }

    pub fn load(&mut self, filename: &str, manager: &Manager) -> Result<Rc<T>, Error> {
        //   self.resources.entry(name.to_string()).or_insert(self.provider.load(name)).clone()
        if let Some(r) = self.resources.get(filename) {
            return Ok(r.clone());
        }

        let r = self.provider.load(filename, manager);

        if let Ok(r) = r {
            self.resources.insert(filename.to_string(), r.clone());
            return Ok(r);
        }

        r
    }
}

impl<T: ?Sized> Cache for TypedCache<T> {}
