pub use self::types::*;
pub mod mock;

use crate::io::result::{Error, Result};
use argon2::Config;
use rand::Rng;

use super::result::ClientError;
mod types;

pub fn hash(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_e| Error::InternalServerError)
}

pub fn verify(hash: &str, attempted_password: &str) -> Result<()> {
    let verified = argon2::verify_encoded(hash, attempted_password.as_bytes())
        .map_err(|_e| Error::InternalServerError)?;

    if verified != true {
        Err(Error::BadRequest(ClientError::authentication_failed()))
    } else {
        Ok(())
    }
}
