use crate::io::error::Error;
use argon2::Config;
use rand::Rng;

pub fn hash(password: &String) -> Result<String, Error> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|e| {
        eprintln!("{}", e);
        Error::InternalServerError
    })
}
