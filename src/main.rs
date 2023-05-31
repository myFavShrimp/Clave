mod app;
use app::{cli_app::ClaveApp, args::get_clap_app};
use simplelog::{CombinedLogger, TermLogger, LevelFilter, TerminalMode, ColorChoice};

use std::path::PathBuf;

fn main() {
    let args = get_clap_app().get_matches();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            ]
        ).unwrap();

        if let Some(file_paths) = args.values_of("files") {
        let mut file_paths: Vec<PathBuf> = file_paths.into_iter()
            .map(PathBuf::from).collect();
        file_paths.sort();
        file_paths.dedup();

        let mut application = ClaveApp { file_paths };
        application.run();
    }
}
