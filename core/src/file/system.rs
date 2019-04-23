use std::fs::File;
use std::io::{BufRead, BufReader};

use error::Error;

pub struct System {
    mount_folders: Vec<String>,
}

impl System {
    pub fn new() -> System {
        System {
            mount_folders: Vec::new(),
        }
    }

    pub fn read_stream(&self, name: &str) -> Result<impl BufRead, Error> {
        let stream = self.open_read_stream(name)?;

        Ok(stream)
    }

    fn open_read_stream(&self, name: &str) -> Result<impl BufRead, Error> {
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

        Err(Error::from_string(
            "Stream \"".to_string() + name + "\" could not be opened",
        ))
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
