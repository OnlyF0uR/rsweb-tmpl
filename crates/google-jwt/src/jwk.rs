use serde::Deserialize;

use crate::base64_decode;
use crate::error::GoogleError;
use crate::structure::Algorithm;

#[derive(Deserialize, Clone, Debug)]
pub struct JsonWebKeySet {
    keys: Vec<JsonWebKey>,
}

impl JsonWebKeySet {
    pub fn get_key(&self, id: &str) -> Option<JsonWebKey> {
        self.keys.iter().find(|key| key.id == id).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonWebKey {
    #[serde(rename = "alg")]
    algorithm: Algorithm,
    #[serde(rename = "kid")]
    id: String,
    n: String,
    e: String,
}

impl JsonWebKey {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> Result<(), GoogleError> {
        match self.algorithm {
            Algorithm::RS256 => {
                let n: Vec<u8> = base64_decode(&self.n)
                    .map_err(|_| GoogleError::InvalidToken("unable decode JWK n (modulus)"))?;
                let e: Vec<u8> = base64_decode(&self.e)
                    .map_err(|_| GoogleError::InvalidToken("unable decode JWK e (exponent)"))?;
                let pubkey = ring::signature::RsaPublicKeyComponents { n, e };
                let res = pubkey.verify(
                    &ring::signature::RSA_PKCS1_2048_8192_SHA256,
                    message,
                    signature_bytes,
                );
                res.map_err(|_err| GoogleError::InvalidToken("invalid token"))?;
                Ok(())
            }
            _ => Err(GoogleError::UnsupportedAlgorithm(self.algorithm)),
        }
    }
}
