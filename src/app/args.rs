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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_argument_parsing() {
        let test_arguments = vec!["clave", "./first_path", "./second_path"];

        let clap_app = get_clap_app();
        let matches = clap_app.get_matches_from(test_arguments);

        let files_matches: Vec<&str> = matches.values_of("files").unwrap().into_iter().collect();
        let files_expected = vec!["./first_path", "./second_path"];

        assert_eq!(
            files_matches,
            files_expected
        );
    }
}
