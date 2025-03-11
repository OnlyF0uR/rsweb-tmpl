use crate::structure::Algorithm;

#[derive(Debug, PartialEq)]
pub enum GoogleError {
    // #[error("Invalid token: {0}")]
    InvalidToken(&'static str),

    // #[error("Failed to decode from Base64: {0}")]
    Base64Error(base64::DecodeError),

    // #[error("Failed to deserialize data: {0}")]
    SerdeError(String),

    // #[error("Failed to retrieve the key")]
    RetrieveKeyFailure,

    // #[error("Unsupported algorithm: {0:?}")]
    UnsupportedAlgorithm(Algorithm),

    // #[error("JWT token has expired")]
    Expired,

    // #[error("Mutex poisoned")]
    MutexPoisoned,
}

impl std::fmt::Display for GoogleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            GoogleError::Base64Error(e) => e.fmt(f),
            GoogleError::SerdeError(msg) => write!(f, "Failed to deserialize data: {}", msg),
            GoogleError::RetrieveKeyFailure => write!(f, "Failed to retrieve the key"),
            GoogleError::UnsupportedAlgorithm(algo) => {
                write!(f, "Unsupported algorithm: {:?}", algo)
            }
            GoogleError::Expired => write!(f, "JWT token has expired"),
            GoogleError::MutexPoisoned => write!(f, "Mutex poisoned"),
        }
    }
}

impl std::error::Error for GoogleError {}

impl From<base64::DecodeError> for GoogleError {
    fn from(_: base64::DecodeError) -> Self {
        GoogleError::InvalidToken("decode error")
    }
}

impl From<serde_json::Error> for GoogleError {
    fn from(err: serde_json::Error) -> Self {
        GoogleError::SerdeError(err.to_string())
    }
}
