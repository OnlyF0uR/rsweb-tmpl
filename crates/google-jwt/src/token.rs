use serde::Deserialize;

pub struct Token<P> {
    pub claims: RequiredClaims,
    pub payload: P,
}

impl<P> Token<P> {
    pub fn new(claims: RequiredClaims, payload: P) -> Token<P> {
        Token { claims, payload }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RequiredClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(rename = "aud")]
    pub audience: String,

    #[serde(rename = "azp")]
    pub android_audience: String,

    #[serde(rename = "iat")]
    pub issued_at: u64,

    #[serde(rename = "exp")]
    pub expires_at: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdPayload {
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub picture: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "hd")]
    pub domain: Option<String>,
}
