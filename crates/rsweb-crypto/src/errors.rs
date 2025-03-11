#[derive(Debug)]
pub enum CryptoError {
    HashError(String),
    ExtractPubkeyError(String),
    SignError(String),
    IoError(std::io::Error),
    NotInitialized,
    FromHexError(hex::FromHexError),
    IncongruentLength(usize, usize),
    ConversionError(std::array::TryFromSliceError),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::HashError(e) => write!(f, "Hash error: {}", e),
            CryptoError::ExtractPubkeyError(e) => write!(f, "Extract public key error: {}", e),
            CryptoError::SignError(e) => write!(f, "Sign error: {}", e),
            CryptoError::IoError(e) => e.fmt(f),
            CryptoError::NotInitialized => write!(f, "KeyStore not initialized"),
            CryptoError::FromHexError(e) => e.fmt(f),
            CryptoError::IncongruentLength(expected, actual) => {
                write!(
                    f,
                    "Incongruent length: expected {}, got {}",
                    expected, actual
                )
            }
            CryptoError::ConversionError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<std::io::Error> for CryptoError {
    fn from(e: std::io::Error) -> Self {
        CryptoError::IoError(e)
    }
}

impl From<hex::FromHexError> for CryptoError {
    fn from(e: hex::FromHexError) -> Self {
        CryptoError::FromHexError(e)
    }
}

impl From<std::array::TryFromSliceError> for CryptoError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        CryptoError::ConversionError(e)
    }
}
