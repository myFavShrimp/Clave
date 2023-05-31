use std::path::PathBuf;

use super::cryptor;

static READ_PASSWORD_ERROR_MESSAGE: &str = "Could not read the password. Trying again ...";
static PASSWORD_MATCH_ERROR_MESSAGE: &str = "The passwords do not match. Trying again ...";

static CHOOSE_PASSWORD_MESSAGE: &str = "Choose a password to use for processing (leave empty to exit): ";
static CONFIRM_PASSWORD_MESSAGE: &str = "Confirm your password: ";

#[derive(Debug)]
pub struct ClaveApp {
    pub file_paths: Vec<PathBuf>,
}

impl ClaveApp {
    pub fn run(&mut self) {
        self.file_paths.sort();
        self.file_paths.dedup();

        println!("These are the paths you have selected for processing:");
        &self.file_paths.iter().for_each(|item| println!("  \"{}\"", item.display()));

        let mut password = match prompt_password() {
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
                    println!("  \"{}\"", file_path.display());
                }
            }
        }
        if encryption_results.iter().any(|item| item.is_err()) {
            println!("Errors occurred during the processing of the following files:");
            for item in &encryption_results {
                if let Err((file_path, error_message)) = item {
                    println!("  {} : \"{}\"", error_message, file_path.display());
                }
            }
        }
    }

}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error reading password")]
    PasswordPromptError(#[from] std::io::Error)
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordPromptError {
    #[error("Error reading password")]
    IoError(#[from] std::io::Error),
    #[error("the passwords do not match")]
    MatchError,
}

fn prompt_password() -> Result<String, PasswordPromptError> {
    let password = rpassword::prompt_password(CHOOSE_PASSWORD_MESSAGE)?;
    let password_confirm = rpassword::prompt_password(CONFIRM_PASSWORD_MESSAGE)?;

    if password == password_confirm {
        Ok(password)
    } else {
        Err(PasswordPromptError::MatchError)
    }
}
