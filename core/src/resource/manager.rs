use super::Cache;
use super::Identifiable;
use file::System as FileSystem;
use std::collections::BTreeMap;

pub struct Manager {
    file_system: FileSystem,

    caches: BTreeMap<&'static str, Box<dyn Cache>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { file_system: FileSystem::new(), caches: BTreeMap::new() }
    }

    pub fn file_system(&mut self) -> &mut FileSystem {
        &mut self.file_system
    }

    pub fn stuff<T: Identifiable>(&self, thing: &T) {
        println!("Things are happening {}", T::identifier());
    }
}
