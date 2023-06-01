use clap;

/// Encrypts your files in place for you to share them securely.
#[derive(clap::Parser, Debug)]
#[command(
    author, 
    version, 
    about, 
    long_about = None, 
    args_conflicts_with_subcommands = true,
    arg_required_else_help(true),
)]
pub struct Args {
    pub paths: Vec<std::path::PathBuf>,
}
