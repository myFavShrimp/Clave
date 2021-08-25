use std::path::PathBuf;

use super::ClaveApp;

/// Clave console application struct.
#[derive(Debug)]
pub struct CliApp {
    file_paths: Vec<PathBuf>,
}

impl ClaveApp for CliApp {
    fn new(file_paths: Vec<PathBuf>) -> Self {
        Self {
            file_paths,
        }
    }

    fn run(&mut self) {
        println!("Running!");
        println!("{:#?}", self.file_paths);
    }
}
