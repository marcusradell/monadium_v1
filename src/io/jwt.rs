use super::error::Error;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        Jwt {
            secret: jwt_secret.into(),
        }
    }

    pub fn encode(&self, email: String) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            // TODO: set exp correctly.
            &Claims { email, exp: 10 },
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(_error: jsonwebtoken::errors::Error) -> Error {
        Error::InternalServerError
    }
}
