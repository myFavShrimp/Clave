use clave::{args::Args, cli_app::ClaveApp};
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use clap::Parser;

fn main() -> Result<(), clave::Error> {
    let args = Args::parse();
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let mut file_paths = args.paths;
    file_paths.sort();
    file_paths.dedup();

    let application = ClaveApp { file_paths };

    if let Err(e) = clave::cli_app::run(&application) {
        log::error!("{}", e);
    }

    Ok(())
}
