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
        self.file_paths.sort();
        self.file_paths.dedup();

        println!("These are the paths you have selected for processing:");
        &self.file_paths.iter().for_each(|item| println!("\"{}\"", item.display()));

        let mut cipher = cryptor::create_cipher("some key".as_bytes());

        let mut encryption_results = vec![];

        for path in &self.file_paths {
            encryption_results.extend(cryptor::encrypt_path(&mut cipher, path));
        }

        println!("Finished!");
        if encryption_results.iter().any(|item| item.is_ok()) {
            println!("The following files were processed successfully:");
            for item in &encryption_results {
                if let Ok(file_path) = item {
                    println!("  {}", file_path.display());
                }
            }
        }
        if encryption_results.iter().any(|item| item.is_ok()) {
            println!("Errors occurred during the processing of the following files:");
            for item in &encryption_results {
                if let Err((file_path, error_message)) = item {
                    println!("  {} : {}", error_message, file_path.display());
                }
            }
        }
    }
}
