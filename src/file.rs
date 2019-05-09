use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct FileHandler {
    path: PathBuf,
}

impl FileHandler {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        FileHandler {
            path: path.as_ref().to_path_buf(),
        }
    }
    pub fn read(&self) -> Vec<String> {
        let mut buffer = String::new();
        let mut file: File = File::open(&self.path).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        buffer.lines().map(|s| s.trim().to_owned()).collect()
    }
    pub fn write(&self, data: Vec<String>) -> std::io::Result<()> {
        let s = data
            .into_iter()
            .map(|l| l + "\n")
            .collect::<Vec<String>>()
            .join("");
        let mut file: File = File::create(&self.path)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }
}
