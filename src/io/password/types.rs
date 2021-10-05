use crate::io::result::Result;

pub trait PasswordHasher {
    fn hash(password: &str) -> Result<String>;
}

pub trait PasswordVerifier {
    fn verify(hash: &str, attempted_password: &str) -> Result<bool>;
}
