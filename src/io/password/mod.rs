use crate::io::result::{Error, Result};
use argon2::Config;
use rand::Rng;

use self::types::{PasswordHasher, PasswordVerifier};
pub mod mock;
pub mod types;

pub struct Password {}

impl PasswordHasher for Password {
    fn hash(self, password: &str) -> Result<String> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|_e| Error::InternalServerError)
    }
}

impl PasswordVerifier for Password {
    fn verify(self, hash: &str, attempted_password: &str) -> Result<bool> {
        argon2::verify_encoded(hash, attempted_password.as_bytes())
            .map_err(|_e| Error::InternalServerError)
    }
}