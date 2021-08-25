pub mod cli_app;

/// Defines how a clave application should look.
pub trait ClaveApp{
    fn new() -> Self;
    fn run(&mut self);
}
