use clave::args::Args;
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use clap::Parser;

fn main() -> Result<(), log::SetLoggerError> {
    let file_paths = Args::parse().paths;
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    if cli_app(file_paths).is_err() {
        std::process::exit(1);
    }

    Ok(())
}

fn cli_app(mut file_paths: Vec<std::path::PathBuf>) -> Result<(), ()> {
    file_paths.sort();
    file_paths.dedup();

    clave::process(file_paths).or(Err(()))?;

    Ok(())
}
