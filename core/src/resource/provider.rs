use super::Identifiable;
use std::rc::Rc;

pub trait Provider<T: ?Sized> {
    fn load(&self, name: &str) -> Rc<T>;
}
