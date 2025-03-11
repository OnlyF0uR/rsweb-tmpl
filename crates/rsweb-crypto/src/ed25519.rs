use base64::{Engine as _, engine::general_purpose};
use nacl::sign::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};
use rand::Rng;
use std::path::Path;
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::sync::OnceCell;

use crate::cast;
use crate::errors::CryptoError;

pub struct KeyStore {
    secret_key: [u8; SECRET_KEY_LENGTH],
    public_key: [u8; PUBLIC_KEY_LENGTH],
}

impl KeyStore {
    async fn initialize() -> Result<Self, CryptoError> {
        let key_path = ".private";

        if Path::new(key_path).exists() {
            let secret_key = async_fs::read(key_path).await?;
            let public_key = match nacl::sign::extract_pkey(&secret_key) {
                Ok(bytes) => bytes,
                Err(err) => return Err(CryptoError::ExtractPubkeyError(err.message)),
            };

            let skey = cast::slice_to_array_64(&secret_key).unwrap();
            let pkey = cast::slice_to_array_32(&public_key).unwrap();

            Ok(KeyStore {
                secret_key: *skey,
                public_key: *pkey,
            })
        } else {
            let random_seed: [u8; 32] = rand::thread_rng().r#gen();
            let kp = nacl::sign::generate_keypair(&random_seed);

            let skey: [u8; 64] = kp.skey;
            let pkey = kp.pkey;

            async_fs::write(key_path, &skey).await?;

            Ok(KeyStore {
                secret_key: skey,
                public_key: pkey,
            })
        }
    }

    pub fn public_key(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.public_key
    }

    pub fn secret_key(&self) -> &[u8; SECRET_KEY_LENGTH] {
        &self.secret_key
    }
}

// Global KeyStore instance
static KEY_STORE_INSTANCE: OnceCell<Arc<KeyStore>> = OnceCell::const_new();

// Function to get a reference to the initialized DB
pub async fn get_key_store() -> Arc<KeyStore> {
    KEY_STORE_INSTANCE
        .get_or_init(|| async {
            let key_store = KeyStore::initialize()
                .await
                .expect("Failed to initialize key store");
            Arc::new(key_store)
        })
        .await
        .clone()
}

pub async fn sign_message(message: &[u8]) -> Vec<u8> {
    let key_store = get_key_store().await;

    match nacl::sign::signature(message, key_store.secret_key()) {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Error: {:?}", err);
            vec![]
        }
    }
}

#[allow(dead_code)]
pub fn sign_message_sync(message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let key_store = KEY_STORE_INSTANCE
        .get()
        .ok_or(CryptoError::NotInitialized)?;

    match nacl::sign::sign(message, key_store.secret_key()) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(CryptoError::SignError(e.message)),
    }
}

#[allow(dead_code)]
pub async fn verify_signature(message: &[u8], signature: &[u8]) -> bool {
    let key_store = get_key_store().await;
    nacl::sign::verify(message, signature, key_store.public_key()).is_ok()
}

pub fn verify_signature_sync(message: &[u8], signature: &[u8]) -> Result<bool, CryptoError> {
    let key_store = KEY_STORE_INSTANCE
        .get()
        .ok_or(CryptoError::NotInitialized)?;

    match nacl::sign::verify(message, signature, key_store.public_key()) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub async fn get_public_key_base64() -> String {
    let key_store = get_key_store().await;
    let vk_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key_store.public_key());
    vk_base64
}

#[allow(dead_code)]
pub fn get_public_key_base64_sync() -> String {
    let key_store = KEY_STORE_INSTANCE.get().expect("KeyStore not initialized");
    let vk_base64 = general_purpose::URL_SAFE_NO_PAD.encode(key_store.public_key());
    vk_base64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_message() {
        let message = b"Hello, world!";
        let signature = sign_message(message).await;
        assert!(!signature.is_empty());
    }

    #[tokio::test]
    async fn test_verify_signature() {
        let message = b"Hello, world!";
        let signature = sign_message(message).await;
        let verified = verify_signature(message, &signature).await;
        assert!(verified);
    }

    #[tokio::test]
    async fn test_get_public_key_base64() {
        let vk_base64 = get_public_key_base64().await;
        assert!(!vk_base64.is_empty());
    }
}
