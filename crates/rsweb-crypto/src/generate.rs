use hex;
use rand::distributions::{Alphanumeric, DistString};
use rand::{Rng, RngCore};

pub fn generate_nonce() -> String {
    let mut rng = rand::thread_rng();
    let n = rng.r#gen::<u32>();

    // Get bytes in little-endian format
    let bytes = n.to_le_bytes();

    hex::encode(bytes)
}

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();

    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    hex::encode(bytes)
}

pub fn generate_name() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 3];
    rng.fill(&mut bytes);

    hex::encode(bytes)
}

pub fn generate_random_string(n: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), n)
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32]; // 32 bytes salt
    rand::thread_rng().fill_bytes(&mut salt); // Fill with random bytes
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), 8);
    }

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert_eq!(id.len(), 32);
    }

    #[test]
    fn test_generate_name() {
        let name = generate_name();
        assert_eq!(name.len(), 6);
    }

    #[test]
    fn test_generate_random_string() {
        let n = 10;
        let random_string = generate_random_string(n);
        assert_eq!(random_string.len(), n);
    }

    #[test]
    fn test_generate_salt() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 32);
    }
}
