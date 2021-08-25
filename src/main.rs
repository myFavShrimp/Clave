mod app;
use app::{cli_app::CliApp, ClaveApp};

fn main() {
    let mut application = CliApp::new();
    application.run();
}
