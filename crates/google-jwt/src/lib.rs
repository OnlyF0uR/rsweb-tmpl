mod client;
mod error;
mod jwk;
mod key_provider;
mod structure;
mod token;
mod unverified_token;

pub use crate::token::{IdPayload, RequiredClaims, Token};
pub use client::*;
pub use error::GoogleError;
pub use key_provider::*;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::Engine as _;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
}
