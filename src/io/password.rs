use crate::io::error::Error;
use argon2::Config;
use rand::Rng;

pub fn hash(password: &String) -> Result<String, Error> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_e| Error::InternalServerError)
}

pub fn verify(hash: &String, attempted_password: &String) -> Result<bool, Error> {
    argon2::verify_encoded(hash, attempted_password.as_bytes())
        .map_err(|_e| Error::InternalServerError)
}
