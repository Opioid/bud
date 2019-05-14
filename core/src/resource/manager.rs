use file::System as FileSystem;
use std::collections::BTreeMap;
use resource::{Identifiable, Cache, TypedCache};
use scene::material::Material;
use scene::material::Provider as MaterialProvider;

pub struct Manager {
    file_system: FileSystem,

    caches: BTreeMap<&'static str, Box<dyn Cache>>,

    material_cache: TypedCache::<dyn Material>
}

impl Manager {
    pub fn new() -> Manager {
        let material_provider = Box::new(MaterialProvider {});
        let material_cache = TypedCache::<Material>::new(material_provider);

        Manager { file_system: FileSystem::new(), caches: BTreeMap::new(), material_cache }
    }

    pub fn file_system(&mut self) -> &mut FileSystem {
        &mut self.file_system
    }

    // pub fn register<T: 'static + Identifiable>(&mut self, cache: Box<TypedCache<T>>) {
    //     self.caches.insert(T::identifier(), cache);
    // }

    pub fn stuff<T: Identifiable>(&self, thing: &T) {
        println!("Things are happening {}", T::identifier());
    }
}
