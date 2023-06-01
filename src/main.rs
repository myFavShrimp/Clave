use clave::{args::Args, cli_app::ClaveApp};
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode};

use clap::Parser;

fn main() -> eyre::Result<(), eyre::Report> {
    let args = Args::parse();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    let mut file_paths = args.paths;
    file_paths.sort();
    file_paths.dedup();

    let application = ClaveApp { file_paths };
    clave::cli_app::run(&application)?;

    Ok(())
}
