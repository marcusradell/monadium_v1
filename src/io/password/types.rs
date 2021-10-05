use crate::io::result::Result;

pub type Hash = fn(password: &str) -> Result<String>;

pub type Verify = fn(hash: &str, attempted_password: &str) -> Result<bool>;
