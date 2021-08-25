use super::ClaveApp;

/// Clave console application struct.
#[derive(Debug)]
pub struct CliApp {
}

impl ClaveApp for CliApp {
    fn new() -> Self {
        Self {}
    }

    fn run(&mut self) {
        println!("Running!")
    }
}
