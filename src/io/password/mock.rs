use super::types::{PasswordHasher, PasswordVerifier};
use crate::io::result::Result;

pub struct PasswordMock {}

impl PasswordHasher for PasswordMock {
    fn hash(_: &str) -> Result<String> {
        Ok("HASHED_BY_MOCK".into())
    }
}

impl PasswordVerifier for PasswordMock {
    fn verify(_: &str, attempted_password: &str) -> Result<bool> {
        match attempted_password {
            "correct_password" => Ok(true),
            _ => Ok(false),
        }
    }
}
