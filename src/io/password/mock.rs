use super::types::{PasswordHasher, PasswordVerifier};
use crate::io::result::Result;

#[derive(Clone)]
pub struct PasswordMock {}

impl PasswordHasher for PasswordMock {
    fn hash(self, _: &str) -> Result<String> {
        Ok("HASHED_BY_MOCK".into())
    }
}

impl PasswordVerifier for PasswordMock {
    fn verify(self, _: &str, attempted_password: &str) -> Result<bool> {
        match attempted_password {
            "correct_password" => Ok(true),
            _ => Ok(false),
        }
    }
}
