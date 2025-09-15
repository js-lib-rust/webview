use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing parameter: {0}")]
    NoParam(String),

    #[error("Incompatible parameter: {0}")]
    BadParam(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
