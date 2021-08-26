use clap::{App, AppSettings, Arg, crate_name, crate_version, crate_authors};

/// Returns the clap app of the application.
pub fn get_clap_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .about("Encrypts your files in place to share them securely.")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("files").value_name("files")
                .help("Files to process").required(true).multiple(true))
}
