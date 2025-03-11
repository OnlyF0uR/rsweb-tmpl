pub mod claims;
pub mod errors;

pub fn google_client_id() -> Option<String> {
    let value = std::env::var("GOOGLE_OAUTH_CLIENT_ID");
    match value {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
