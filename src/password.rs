static CHOOSE_PASSWORD_MESSAGE: &str = "Choose a password to use for processing: ";
static CONFIRM_PASSWORD_MESSAGE: &str = "Confirm your password: ";

#[derive(Debug, thiserror::Error)]
pub enum PasswordPromptError {
    #[error("Error reading password - {0}")]
    IoError(#[from] std::io::Error),
    #[error("the passwords do not match")]
    MatchError,
}

pub fn prompt_password() -> Result<String, PasswordPromptError> {
    let password = rpassword::prompt_password(CHOOSE_PASSWORD_MESSAGE)?;
    let password_confirm = rpassword::prompt_password(CONFIRM_PASSWORD_MESSAGE)?;

    if password == password_confirm {
        Ok(password)
    } else {
        Err(PasswordPromptError::MatchError)
    }
}
