use resource;
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

    pub fn load(&mut self, name: &str) -> Rc<T> {
        self.resources.entry(name.to_string()).or_insert(self.provider.load(name)).clone()
    }
}
