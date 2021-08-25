pub mod cli_app;
pub mod args;

use std::path::PathBuf;

/// Defines how a clave application should look.
pub trait ClaveApp{
    fn new(file_paths: Vec<PathBuf>) -> Self;
    fn run(&mut self);
}
