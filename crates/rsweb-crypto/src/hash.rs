use crate::errors::CryptoError;

fn hash_progress_callback(progress: u32) {
    eprintln!("Hashing progress: {}%", progress);
}

pub fn hash_password(password: &[u8]) -> Result<(String, String), CryptoError> {
    let salt_bytes = super::generate::generate_salt();
    let digest = match nacl::scrypt(
        password,
        &salt_bytes,
        10, // u8::pow(2, 15),
        8,
        16,
        64,
        &hash_progress_callback,
    ) {
        Ok(d) => d,
        Err(e) => {
            return Err(CryptoError::HashError(e.message));
        }
    };

    Ok((hex::encode(digest), hex::encode(salt_bytes)))
}

pub fn cmp_password_hash(
    input_pwd: &str,
    cmp_hash: &str,
    cmp_salt: &str,
) -> Result<bool, CryptoError> {
    let salt_bytes = hex::decode(cmp_salt)?;
    let digest = match nacl::scrypt(
        input_pwd.as_bytes(),
        &salt_bytes,
        10, // u8::pow(2, 15),
        8,
        16,
        64,
        &hash_progress_callback,
    ) {
        Ok(d) => d,
        Err(e) => {
            return Err(CryptoError::HashError(e.message));
        }
    };

    let hash_bytes = hex::decode(cmp_hash)?;
    Ok(nacl::compare(&digest, &hash_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password";
        let (hash, salt) = hash_password(password.as_bytes()).unwrap();
        assert_eq!(hash.len(), 128);
        assert_eq!(salt.len(), 64);
    }

    #[test]
    fn test_cmp_password_hash() {
        let password = "password";
        let (hash, salt) = hash_password(password.as_bytes()).unwrap();
        assert_eq!(cmp_password_hash(password, &hash, &salt).unwrap(), true);
    }
}
