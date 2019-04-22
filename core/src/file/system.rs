use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

pub struct System {
    mount_folders: Vec<String>,
}

impl System {
    pub fn new() -> System {
        System{mount_folders: Vec::new()}
    }

    pub fn read_stream(&self, name: &str) -> io::Result<impl BufRead> {
        let stream = self.open_read_stream(name)?;

        Ok(stream)
    }

    fn open_read_stream(&self, name: &str) -> io::Result<impl BufRead> {
        for f in self.mount_folders.iter() {
            if f.is_empty() {
                continue;
            }

            let resolved_name = f.to_string() + name;

            if let Ok(file) = File::open(resolved_name) {
                return Ok(BufReader::new(file));
            }
        }

        if let Ok(file) = File::open(name) {
            return Ok(BufReader::new(file));
        }
        
        Err(Error::new(ErrorKind::Other, "Nothing found!!"))
    }
    
    pub fn push_mount(&mut self, folder: &str) {
        // We also have to push empty folders, otherwise popping gets complicated

        let mut folder = folder.to_string();

        if !folder.is_empty() && folder.chars().last().unwrap() == '/' {
            folder += "/";
        }

        self.mount_folders.push(folder);
    }

    pub fn pop_mount(&mut self) {
        self.mount_folders.pop();
    }
}
