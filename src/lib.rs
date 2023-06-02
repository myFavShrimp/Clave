pub mod args;
mod cryptor;
mod hash;
mod password;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error setting up logging output")]
    LoggerError(#[from] log::SetLoggerError),
    #[error("error reading password: {0}")]
    PasswordPromptError(#[from] password::PasswordPromptError),
    #[error("error collecting arguments: {0}")]
    ClapError(#[from] clap::Error),
}

pub fn process(paths: Vec<std::path::PathBuf>) -> Result<(), Error> {
    let password = password::prompt_password()?;

    let mut cipher = cryptor::create_cipher(password.as_bytes());

    let result = paths.iter().fold(
        cryptor::FinalEncryptionResult::default(),
        |mut acc, path| {
            acc.extend(cryptor::encrypt_path(&mut cipher, path));
            acc
        },
    );

    dbg!("{}", result);

    Ok(())
}
