use std::path::PathBuf;

use super::ClaveApp;
use super::cryptor;

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

        self.file_paths.sort();
        self.file_paths.dedup();

        let mut cipher = cryptor::create_cipher("some key".as_bytes());

        for path in &self.file_paths {
            println!("{:?}", cryptor::encrypt_path(&mut cipher, path));
        }
    }
}
