use crate::io::result::Result;

pub trait PasswordHasher {
    fn hash(self, password: &str) -> Result<String>;
}

pub trait PasswordVerifier {
    fn verify(self, hash: &str, attempted_password: &str) -> Result<bool>;
}
