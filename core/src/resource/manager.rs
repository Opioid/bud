use error::Error;
use file::System as FileSystem;
use resource::{Cache, Identifiable, TypedCache};
use scene::material::Material;
use scene::material::Provider as MaterialProvider;
use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct Manager {
    file_system: RefCell<FileSystem>,

    caches: BTreeMap<&'static str, Box<dyn Cache>>,

    material_cache: RefCell<TypedCache<dyn Material>>,
}

impl Manager {
    pub fn new() -> Manager {
        let material_provider = Box::new(MaterialProvider {});
        let material_cache = RefCell::new(TypedCache::<Material>::new(material_provider));

        Manager {
            file_system: RefCell::new(FileSystem::new()),
            caches: BTreeMap::new(),
            material_cache,
        }
    }

    pub fn file_system(&self) -> RefMut<FileSystem> {
        self.file_system.borrow_mut()
    }

    // pub fn register<T: 'static + Identifiable>(&mut self, cache: Box<TypedCache<T>>) {
    //     self.caches.insert(T::identifier(), cache);
    // }

    pub fn load_material(&self, filename: &str) -> Result<Rc<dyn Material>, Error> {
        //   Err(Error::new("No material implemented yet"))
        self.material_cache.borrow_mut().load(filename, self)
    }

    pub fn stuff<T: Identifiable>(&self, thing: &T) {
        println!("Things are happening {}", T::identifier());
    }
}
