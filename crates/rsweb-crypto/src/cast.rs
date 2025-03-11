use crate::errors::CryptoError;

pub fn slice_to_array_32<T>(slice: &[T]) -> Result<&[T; 32], CryptoError> {
    if slice.len() == 32 {
        let ptr = slice.as_ptr() as *const [T; 32];
        unsafe { Ok(&*ptr) }
    } else {
        Err(CryptoError::IncongruentLength(32, slice.len()))
    }
}

pub fn slice_to_array_64<T>(slice: &[T]) -> Result<&[T; 64], CryptoError> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 64];
        unsafe { Ok(&*ptr) }
    } else {
        Err(CryptoError::IncongruentLength(32, slice.len()))
    }
}

pub fn vec_to_array_32(vec: Vec<u8>) -> Result<[u8; 32], CryptoError> {
    if vec.len() != 32 {
        return Err(CryptoError::IncongruentLength(32, vec.len()));
    }

    let array: [u8; 32] = vec.as_slice().try_into()?;
    Ok(array)
}

pub fn vec_to_array_64(vec: Vec<u8>) -> Result<[u8; 64], CryptoError> {
    if vec.len() != 64 {
        return Err(CryptoError::IncongruentLength(64, vec.len()));
    }

    let array: [u8; 64] = vec.as_slice().try_into()?;
    Ok(array)
}

pub fn buffer_to_hex(buffer: &[u8]) -> String {
    buffer.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_to_array_32() {
        let slice = vec![0u8; 32];
        let array = slice_to_array_32(&slice).unwrap();
        assert_eq!(array.len(), 32);
    }

    #[test]
    fn test_slice_to_array_64() {
        let slice = vec![0u8; 64];
        let array = slice_to_array_64(&slice).unwrap();
        assert_eq!(array.len(), 64);
    }

    #[test]
    fn test_vec_to_array_32() {
        let vec = vec![0u8; 32];
        let array = vec_to_array_32(vec).unwrap();
        assert_eq!(array.len(), 32);
    }

    #[test]
    fn test_vec_to_array_64() {
        let vec = vec![0u8; 64];
        let array = vec_to_array_64(vec).unwrap();
        assert_eq!(array.len(), 64);
    }

    #[test]
    fn test_buffer_to_hex() {
        let buffer = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let hex = buffer_to_hex(&buffer);
        assert_eq!(hex, "00010203040506070809");
    }
}
