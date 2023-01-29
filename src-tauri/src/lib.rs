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
    DecryptionError(String),
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

pub fn write_token(token: &str, path: &str) -> Result<(), StorageError> {
    let encrypted_token = encrypt_token(token)?;
    fs::write(path, encrypted_token)
        .map_err(|_| StorageError::EncryptionError("Unable to write token to file".into()))?;

    Ok(())
}

#[cfg(test)]
mod test_write_token {
    use std::{env, fs};

    use crate::write_token;
    #[test]
    fn success() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let res = write_token("hello", ".testtoken");
        assert!(res.is_ok());

        let got = fs::read(".testtoken");
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
    fn fail_if_key_and_nonce_invalid() {
        env::remove_var("FILE_ENCRYPTION_KEY");
        let got = encrypt_token("hello");
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to encrypt data: FILE_ENCRYPTION_KEY environment variable is not set"
        );
    }

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

    #[test]
    fn succeed_on_empty_token() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");
        let got = encrypt_token("");
        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(
            got,
            vec![124, 126, 210, 66, 163, 145, 204, 27, 165, 192, 253, 65, 185, 90, 206, 65]
        );
    }
}

pub fn read_token(path: &str) -> Result<String, StorageError> {
    let encrypted_data = fs::read(path)
        .map_err(|_| StorageError::DecryptionError("Unable to read data from file".into()))?;

    let decrypted_token = decrypt_token(encrypted_data)?;

    Ok(decrypted_token)
}

#[cfg(test)]
mod test_read_token {
    use std::{env, fs};

    use crate::read_token;

    #[test]
    fn success() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        fs::write(
            ".testfile",
            vec![
                145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165,
                93, 198, 108, 208,
            ],
        )
        .unwrap();

        let got = read_token(".testfile");

        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(got, "hello");

        fs::remove_file(".testfile").ok();
    }

    #[test]
    fn fail_on_no_env_variables() {
        env::remove_var("FILE_ENCRYPTION_KEY");
        fs::write(
            ".testfile",
            vec![
                145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165,
                93, 198, 108, 208,
            ],
        )
        .unwrap();

        let got = read_token(".testfile");
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to encrypt data: FILE_ENCRYPTION_KEY environment variable is not set"
        );

        fs::remove_file(".testfile").ok();
    }

    #[test]
    fn fail_on_invalid_contents() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        fs::write(".testfile", vec![]).unwrap();

        let got = read_token(".testfile");
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(err.to_string(), "Unable to decrypt data: aead::Error");

        fs::remove_file(".testfile").ok();
    }

    #[test]
    fn fail_if_file_absent() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let got = read_token(".nonexistentfile");
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to decrypt data: Unable to read data from file"
        );
    }
}

pub fn decrypt_token(encrypted_token: Vec<u8>) -> Result<String, StorageError> {
    let (key, nonce): ([u8; 32], [u8; 24]) = get_key_and_nonce()?;

    let cipher = XChaCha20Poly1305::new(&key.into());
    let decrypted_token = cipher
        .decrypt(&nonce.into(), encrypted_token.as_ref())
        .map_err(|e| StorageError::DecryptionError(e.to_string()))?;

    let token = String::from_utf8(decrypted_token).map_err(|_| {
        StorageError::DecryptionError("Unable to read token from derypted data".into())
    })?;

    Ok(token)
}

#[cfg(test)]
mod test_decrypt_token {
    use std::env;

    use crate::decrypt_token;

    #[test]
    fn fail_if_key_and_nonce_invalid() {
        env::remove_var("FILE_ENCRYPTION_KEY");
        let got = decrypt_token(vec![
            145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165, 93,
            198, 108, 208,
        ]);
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to encrypt data: FILE_ENCRYPTION_KEY environment variable is not set"
        );
    }

    #[test]
    fn success() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let got = decrypt_token(vec![
            145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165, 93,
            198, 108, 208,
        ]);

        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(got, "hello");
    }

    #[test]
    fn fail_on_empty_token() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let got = decrypt_token(vec![]);
        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(err.to_string(), "Unable to decrypt data: aead::Error");
    }
}

pub fn get_key_and_nonce() -> Result<([u8; 32], [u8; 24]), StorageError> {
    let key: String = env::var("FILE_ENCRYPTION_KEY").map_err(|_| {
        StorageError::EncryptionError("FILE_ENCRYPTION_KEY environment variable is not set".into())
    })?;
    let key_array: [u8; 32] = key
        .as_bytes()
        .try_into()
        .map_err(|_| StorageError::KeyConversionError)?;
    let nonce = env::var("FILE_ENCRYPTION_NONCE").map_err(|_| {
        StorageError::EncryptionError(
            "FILE_ENCRYPTION_NONCE environment variable is not set".into(),
        )
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

    use crate::get_key_and_nonce;

    #[test]
    fn no_key() {
        env::remove_var("FILE_ENCRYPTION_KEY");
        let got = get_key_and_nonce();

        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to encrypt data: FILE_ENCRYPTION_KEY environment variable is not set"
        );
    }

    #[test]
    fn key_invalid_u8() {
        env::set_var("FILE_ENCRYPTION_KEY", "��������������������������������");
        let got = get_key_and_nonce();

        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Key must be a string of 32 characters with utf-8 formatting"
        );
    }

    #[test]
    fn no_nonce() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::remove_var("FILE_ENCRYPTION_NONCE");

        let got = get_key_and_nonce();

        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Unable to encrypt data: FILE_ENCRYPTION_NONCE environment variable is not set"
        );
    }

    #[test]
    fn nonce_invalid_u8() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "������������������������");
        let got = get_key_and_nonce();

        assert!(got.is_err());

        let err = got.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Nonce must be a string of 24 characters with utf-8 formatting"
        );
    }

    #[test]
    fn valid_key_and_nonce() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let got = get_key_and_nonce();
        assert!(got.is_ok());

        let (key, nonce) = got.unwrap();
        assert_eq!(
            key,
            [
                49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49,
                50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50
            ],
        );
        assert_eq!(
            nonce,
            [
                49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49,
                50, 51, 52
            ]
        );
    }
}

#[cfg(test)]
mod test_round_trip {
    use crate::{read_token, write_token};
    use std::{env, fs};

    #[test]

    fn test_round_trip() {
        env::set_var("FILE_ENCRYPTION_KEY", "12345678901234567890123456789012");
        env::set_var("FILE_ENCRYPTION_NONCE", "123456789012345678901234");

        let res = write_token("hello", ".testtoken");
        assert!(res.is_ok());

        let got = fs::read(".testtoken");
        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(
            got,
            vec![
                145, 197, 231, 192, 14, 9, 75, 168, 206, 124, 218, 212, 108, 206, 158, 140, 165,
                93, 198, 108, 208
            ]
        );

        let got = read_token(".testtoken");
        assert!(got.is_ok());

        let got = got.unwrap();
        assert_eq!(got, "hello");
    }
}
