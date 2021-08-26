use std::path::PathBuf;

use super::ClaveApp;
use super::cryptor;

use rpassword::read_password_from_tty;

/// Clave console application struct.
#[derive(Debug)]
pub struct CliApp {
    file_paths: Vec<PathBuf>,
}

impl ClaveApp for CliApp {
    fn new(file_paths: Vec<PathBuf>) -> Self {
        Self {
            file_paths,
        }
    }

    fn run(&mut self) {
        self.file_paths.sort();
        self.file_paths.dedup();

        println!("These are the paths you have selected for processing:");
        &self.file_paths.iter().for_each(|item| println!("\"{}\"", item.display()));

        let mut password = match CliApp::prompt_password() {
            Some(password) => { password }
            _ => {
                println!("No input. Application is exited.");
                std::process::exit(0);
            }
        };

        let mut cipher = cryptor::create_cipher(password.as_bytes());
        let mut encryption_results = vec![];

        for path in &self.file_paths {
            encryption_results.extend(cryptor::encrypt_path(&mut cipher, path));
        }

        println!("Finished!");
        if encryption_results.iter().any(|item| item.is_ok()) {
            println!("The following files were processed successfully:");
            for item in &encryption_results {
                if let Ok(file_path) = item {
                    println!("  {}", file_path.display());
                }
            }
        }
        if encryption_results.iter().any(|item| item.is_ok()) {
            println!("Errors occurred during the processing of the following files:");
            for item in &encryption_results {
                if let Err((file_path, error_message)) = item {
                    println!("  {} : {}", error_message, file_path.display());
                }
            }
        }
    }
}

const READ_PASSWORD_ERROR_MESSAGE: &'static str = "Could not read the password. Trying again ...";
const PASSWORD_MATCH_ERROR_MESSAGE: &'static str = "The passwords do not match. Trying again ...";

const CHOOSE_PASSWORD_MESSAGE: &'static str
    = "Choose a password to use for processing (leave empty to exit): ";
const CONFIRM_PASSWORD_MESSAGE: &'static str
    = "Confirm your password: ";

impl CliApp {
    /// Prompts the user to choose a password and reads the users response.
    fn prompt_password() -> Option<String> {
        let mut password: Option<String> = None;

        while password.is_none() {
            match read_password_from_tty(Some(CHOOSE_PASSWORD_MESSAGE)) {
                Ok(input) => {
                    if !&input.is_empty() {
                        match read_password_from_tty(Some(CONFIRM_PASSWORD_MESSAGE)) {
                            Ok(input_confirm) => {
                                if input == input_confirm {
                                    password = Some(input);
                                }
                                else {
                                    println!("{}", PASSWORD_MATCH_ERROR_MESSAGE);
                                }
                            }
                            _ => { println!("{}", READ_PASSWORD_ERROR_MESSAGE); }
                        }
                    }
                    else {
                        break;
                    }
                }
                _ => { println!("{}", READ_PASSWORD_ERROR_MESSAGE); }
            }
        }
        password
    }
}
