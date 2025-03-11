#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    DecodeError(base64::DecodeError),
    JsonError(serde_json::Error),
    TokenExpired,
    InvalidSignature,
    StandardError(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::DecodeError(e) => e.fmt(f),
            AuthError::JsonError(e) => e.fmt(f),
            AuthError::TokenExpired => write!(f, "Token expired"),
            AuthError::InvalidSignature => write!(f, "Invalid signature"),
            AuthError::StandardError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<base64::DecodeError> for AuthError {
    fn from(e: base64::DecodeError) -> Self {
        AuthError::DecodeError(e)
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(e: serde_json::Error) -> Self {
        AuthError::JsonError(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AuthError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AuthError::StandardError(e.to_string())
    }
}
