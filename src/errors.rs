use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellGptError {
    #[error("Could not find the OpenAI API key")]
    ApiKeyMissing,
    #[error("The provided OpenAI API key is an empty string")]
    ApiKeyEmpty,
    #[error("There was an error when calling the OpenAI API: {0}")]
    ApiError(String),

    #[error("Unknown error")]
    Unknown,
}
