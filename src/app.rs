pub mod cli_app;
pub mod args;
mod cryptor;

use std::path::PathBuf;

/// Defines what a clave application should look like.
pub trait ClaveApp{
    fn new(file_paths: Vec<PathBuf>) -> Self;
    fn run(&mut self);
}
