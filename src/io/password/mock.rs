use crate::io::result::Result;

pub fn hash(_: &str) -> Result<String> {
    Ok("HASHED_BY_MOCK".into())
}

pub fn verify(_: &str, attempted_password: &str) -> Result<bool> {
    match attempted_password {
        "correct_password" => Ok(true),
        _ => Ok(false),
    }
}
