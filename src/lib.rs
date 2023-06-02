use cryptor::FinalEncryptionResult;

pub mod args;
mod cryptor;
mod hash;
mod password;

pub fn process(
    paths: Vec<std::path::PathBuf>,
) -> Result<FinalEncryptionResult, password::PasswordPromptError> {
    let password = password::prompt_password().or_else(|e| {
        log::error!("{}", e);
        Err(e)
    })?;

    let mut cipher = cryptor::create_cipher(password.as_bytes());

    Ok(paths.iter().fold(
        cryptor::FinalEncryptionResult::default(),
        |mut acc, path| {
            acc.extend(cryptor::encrypt_path(&mut cipher, path));
            acc
        },
    ))
}
