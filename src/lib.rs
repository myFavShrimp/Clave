use cli_app::PasswordPromptError;

pub mod args;
pub mod cli_app;
mod cryptor;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error setting up logging output")]
    LoggerError(#[from] log::SetLoggerError),
    #[error("error reading password: {0}")]
    PasswordPromptError(#[from] PasswordPromptError),
}
