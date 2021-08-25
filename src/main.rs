mod app;
use app::{cli_app::CliApp, ClaveApp, args::get_clap_app};

use std::path::PathBuf;

fn main() {
    let args = get_clap_app().get_matches();
        if let Some(file_paths) = args.values_of("files") {
        let file_paths: Vec<PathBuf> = file_paths.into_iter()
            .map(|entry| {
                PathBuf::from(entry)
            }).collect();

        let mut application = CliApp::new(file_paths);
        application.run();
    }
}
