use std::{env, fs};

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
};
use serde::Serialize;
use thiserror::Error;

pub const TOKEN_PATH: &str = ".token";

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Unable to encrypt data: {0}")]
    EncryptionError(String),
    #[error("Unable to decrypt data: {0}")]
    DecryptionEror(String),
    #[error("Key must be a string of 32 characters with utf-8 formatting")]
    KeyConversionError,
    #[error("Nonce must be a string of 24 characters with utf-8 formatting")]
    NonceConversionError,
}

impl Serialize for StorageError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub fn write_token(token: &str) -> Result<(), StorageError> {
    let encrypted_token = encrypt_token(token)?;
    fs::write(TOKEN_PATH, encrypted_token)
        .map_err(|_| StorageError::EncryptionError("Unable to write token to file".into()))?;

    Ok(())
}

#[cfg(test)]
mod test_write_token {
    #[test]
    fn success() {}

    #[test]
    fn fail() {}
}

// When refresh tokens are implemented - add those to be read too
pub fn encrypt_token(token: &str) -> Result<Vec<u8>, StorageError> {
    let (key, nonce): ([u8; 32], [u8; 24]) = get_key_and_nonce()?;

    let cipher = XChaCha20Poly1305::new(&key.into());
    let encrypted_vec = cipher
        .encrypt(&nonce.into(), token.as_bytes().as_ref())
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;

    Ok(encrypted_vec)
}

#[cfg(test)]
mod test_encrypt_token {
    use std::env;

    use crate::encrypt_token;

    #[test]
    fn fail_if_key_and_nonce_invalid() {}

    #[test]
    fn success() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");
        let got = encrypt_token("hello");
        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(
            got,
            vec![
                145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165,
                93, 198, 108, 208
            ]
        );
    }

    // Is there a limit to how large the tokens can be?
    #[test]
    fn fail_on_large_token() {}

    // What is the limit?
    #[test]
    fn succeed_on_empty_token() {}
}

pub fn read_token() -> Result<String, StorageError> {
    let encrypted_data = fs::read(TOKEN_PATH)
        .map_err(|_| StorageError::DecryptionEror("Unable to read data from file".into()))?;

    let decrypted_token = decrypt_token(encrypted_data)?;

    Ok(decrypted_token)
}

#[cfg(test)]
mod test_read_token {
    #[test]
    fn success() {}

    #[test]
    fn fail() {}

    #[test]
    fn fail_if_file_absent() {}
}

pub fn decrypt_token(encrypted_token: Vec<u8>) -> Result<String, StorageError> {
    let (key, nonce): ([u8; 32], [u8; 24]) = get_key_and_nonce()?;

    let cipher = XChaCha20Poly1305::new(&key.into());
    let decrypted_token = cipher
        .decrypt(&nonce.into(), encrypted_token.as_ref())
        .map_err(|e| StorageError::DecryptionEror(e.to_string()))?;

    let token = String::from_utf8(decrypted_token).map_err(|_| {
        StorageError::DecryptionEror("Unable to read token from derypted data".into())
    })?;

    Ok(token)
}

#[cfg(test)]
mod test_decrypt_token {
    #[test]
    fn fail_if_key_and_nonce_invalid() {}

    #[test]
    fn success() {}

    // Is there a limit to how large the tokens can be?
    #[test]
    fn fail_on_large_token() {}

    // What is the limit?
    #[test]
    fn succeed_on_empty_token() {}
}

pub fn get_key_and_nonce() -> Result<([u8; 32], [u8; 24]), StorageError> {
    let key: String = env::var("FILE_ENCRYPTION_KEY").map_err(|_| {
        StorageError::EncryptionError("No encryption key environment variable".into())
    })?;
    let key_array: [u8; 32] = key
        .as_bytes()
        .try_into()
        .map_err(|_| StorageError::KeyConversionError)?;
    let nonce = env::var("FILE_ENCRYPTION_NONCE").map_err(|_| {
        StorageError::EncryptionError("No encryption nonce environment variable".into())
    })?;
    let nonce_array: [u8; 24] = nonce
        .as_bytes()
        .try_into()
        .map_err(|_| StorageError::NonceConversionError)?;
    Ok((key_array, nonce_array))
}

#[cfg(test)]
mod test_get_key_and_nonce {
    use std::env;

    #[test]
    fn no_key() {
        env::remove_var("FILE_ENCRYPTION_KEY");
    }

    #[test]
    fn key_invalid_u8() {
        env::set_var("FILE_ENCRYPTION_KEY", "value")
    }

    #[test]
    fn no_nonce() {
        env::remove_var("FILE_ENCRYPTION_NONCE")
    }

    #[test]
    fn nonce_invalid_u8() {
        env::set_var("FILE_ENCRYPTION_NONCE", "")
    }

    #[test]
    fn valid_key_and_nonce() {
        //
    }
}
