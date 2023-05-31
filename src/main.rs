use clave::{args::get_clap_app, cli_app::ClaveApp};
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode};

use std::path::PathBuf;

fn main() -> eyre::Result<(), eyre::Report> {
    let args = get_clap_app().get_matches();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Warn,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    let file_paths = args.values_of("files").unwrap();
    let mut file_paths: Vec<PathBuf> = file_paths.into_iter().map(PathBuf::from).collect();
    file_paths.sort();
    file_paths.dedup();

    let mut application = ClaveApp { file_paths };
    application.run()?;

    Ok(())
}
