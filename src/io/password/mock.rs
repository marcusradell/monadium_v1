#![allow(dead_code)]

use crate::io::result::{ClientError, Error, Result};

pub fn hash(_: &str) -> Result<String> {
    Ok("HASHED_BY_MOCK".into())
}

pub fn verify(_: &str, attempted_password: &str) -> Result<()> {
    match attempted_password {
        "correct_password" => Ok(()),
        _ => Err(Error::BadRequest(ClientError::authentication_failed())),
    }
}
