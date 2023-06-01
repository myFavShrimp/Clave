use std::path::PathBuf;

use super::cryptor;

static CHOOSE_PASSWORD_MESSAGE: &str =
    "Choose a password to use for processing (leave empty to exit): ";
static CONFIRM_PASSWORD_MESSAGE: &str = "Confirm your password: ";

#[derive(Debug)]
pub struct ClaveApp {
    pub file_paths: Vec<PathBuf>,
}

pub fn run(state: &ClaveApp) -> Result<(), crate::Error> {
    let mut message = String::from("These are the paths you have selected for processing:");
    for path in &state.file_paths {
        message.push_str(&format!("\n  \"{}\"", path.display()));
    }

    log::info!("{}", message);

    let password = prompt_password()?;

    let mut cipher = cryptor::create_cipher(password.as_bytes());
    let mut encryption_results = vec![];

    for path in &state.file_paths {
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
    };

    Ok(())
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
