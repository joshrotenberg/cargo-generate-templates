use thiserror::Error;
use serde::Deserialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API error: {0}")]
    APIError(ApiError),
    #[error("Unknown error")]
    Unknown,
}
#[derive(Debug, Deserialize, Clone)]
#[non_exhaustive]
pub struct ApiError {
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
