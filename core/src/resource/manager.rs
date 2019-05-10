use file::System as FileSystem;

pub struct Manager {
    file_system: FileSystem,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { file_system: FileSystem::new() }
    }

    pub fn file_system(&mut self) -> &mut FileSystem {
        &mut self.file_system
    }
}
