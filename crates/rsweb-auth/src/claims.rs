use base64::{Engine as _, engine::general_purpose};
use rsweb_database::user::UserEssentials;
use serde::{Deserialize, Serialize};

use crate::errors::AuthError;

#[derive(Debug)]
pub struct AuthSession {
    pub claims: Claims,
    pub updated_tokens: Option<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: i32,
    #[serde(rename = "e")]
    pub email: String,
    #[serde(rename = "n")]
    pub username: String,
    #[serde(rename = "r")]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "a")]
    pub agency_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenMeta {
    #[serde(rename = "c")]
    claims: Claims,
    #[serde(rename = "exp")]
    expires: i64,
    #[serde(rename = "n")]
    nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignatureToken {
    #[serde(rename = "m")]
    meta: TokenMeta,
    #[serde(rename = "d")]
    digest: String,
}

impl Claims {
    pub async fn try_from_tokens(
        auth_token: &Option<String>,
        refresh_token: &Option<String>,
    ) -> Result<(Self, Option<(String, String)>), AuthError> {
        let auth_token = match auth_token {
            Some(token) => token,
            None => return Err(AuthError::InvalidToken),
        };

        let json_str = general_purpose::URL_SAFE_NO_PAD.decode(auth_token.as_bytes())?;
        let sig_token = serde_json::from_slice::<SignatureToken>(&json_str)?;

        // Check signature
        let json = serde_json::to_string(&sig_token.meta.claims)?;
        let sbytes = general_purpose::URL_SAFE_NO_PAD.decode(sig_token.digest.as_bytes())?;

        if !rsweb_crypto::ed25519::verify_signature(json.as_bytes(), sbytes.as_slice()).await {
            return Err(AuthError::InvalidSignature);
        }

        let mut updated_tokens = None;

        // Check expiry
        if sig_token.meta.expires < unix_secs() {
            if let Some(refresh_token) = refresh_token {
                match refresh_tokens::rotate(refresh_token).await {
                    Ok(tokens) => {
                        updated_tokens = Some(tokens);
                    }
                    Err(_) => return Err(AuthError::TokenExpired),
                };
            } else {
                return Err(AuthError::TokenExpired);
            }
        }

        Ok((sig_token.meta.claims, updated_tokens))
    }

    pub async fn from_user_essentials(user_essentials: &UserEssentials) -> Self {
        Claims {
            uid: user_essentials.id,
            email: user_essentials.email.clone(),
            username: user_essentials.handle.clone(),
            role: user_essentials.role.clone(),
            agency_id: None, // TODO: Implement agency
        }
    }

    pub async fn create_token(&self) -> String {
        let expiry = unix_secs() + 7200;
        let nonce = rsweb_crypto::generate::generate_nonce();

        let meta = TokenMeta {
            claims: self.clone(),
            expires: expiry,
            nonce,
        };

        let meta_str = serde_json::to_string(&meta).unwrap();
        let meta_bytes = meta_str.as_bytes();

        let sig = rsweb_crypto::ed25519::sign_message(meta_bytes).await;
        let digest = general_purpose::URL_SAFE_NO_PAD.encode(sig);

        let access_token = SignatureToken { meta, digest };
        let access_token_str = serde_json::to_string(&access_token).unwrap();

        general_purpose::URL_SAFE_NO_PAD.encode(access_token_str.as_bytes())
    }

    pub fn has_creator_privilege(&self) -> bool {
        self.role == "creator" || self.role == "admin+" || self.role == "agency"
    }
}

pub mod refresh_tokens {
    use rsweb_database::user::UserService;

    use crate::errors::AuthError;

    use super::Claims;

    pub async fn create(user_id: i32) -> Result<String, AuthError> {
        let token = rsweb_crypto::generate::generate_random_string(32);

        UserService::insert_user_refresh_token(user_id, &token).await?;
        Ok(token)
    }

    // TODO: We could also take in a reference to claims
    // and change the query so it doesnt get the user essentials since
    // we kind of already have those
    pub async fn rotate(cookie_rt_str: &str) -> Result<(String, String), AuthError> {
        // Check if the refresh token is valid
        let us = UserService::get_user_essentials_by_refresh_token(cookie_rt_str).await?;

        let at = Claims::from_user_essentials(&us).await.create_token().await;
        let rt = rsweb_crypto::generate::generate_random_string(32);

        UserService::delete_user_refresh_token(us.id).await?;
        UserService::insert_user_refresh_token(us.id, &rt).await?;
        Ok((at, rt))
    }
}

pub fn unix_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
