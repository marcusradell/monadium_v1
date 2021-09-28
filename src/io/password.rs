use crate::io::result::{Error, Result};
use argon2::Config;
use rand::Rng;

pub fn hash(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_e| Error::InternalServerError)
}

#[allow(dead_code)]
pub fn hash_mock(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let mut config = Config::default();

    config.time_cost = 1;

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_e| Error::InternalServerError)
}

pub fn verify(hash: &str, attempted_password: &str) -> Result<bool> {
    argon2::verify_encoded(hash, attempted_password.as_bytes())
        .map_err(|_e| Error::InternalServerError)
}
