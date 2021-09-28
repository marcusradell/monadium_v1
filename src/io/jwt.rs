use crate::io::result::ClientError;

use super::result::Error;
use chrono::offset::Utc;
use jsonwebtoken::{
    errors as jwt_errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub role: String,
    pub email: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
}

impl Jwt {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        Jwt {
            secret: jwt_secret.into(),
        }
    }

    pub fn from_secret(secret: &str) -> Self {
        Jwt {
            secret: secret.to_string(),
        }
    }

    pub fn encode(
        &self,
        id: &Uuid,
        role: &str,
        email: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                id: *id,
                role: role.into(),
                email: email.into(),
                exp: Utc::now().timestamp() + 15 * 60,
            },
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    pub fn decode(&self, encoded_jwt: String) -> Result<Claims, Error> {
        let result: TokenData<Claims> = jsonwebtoken::decode(
            &encoded_jwt,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::default()),
        )?;

        Ok(result.claims.into())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Error {
        match *error.kind() {
            jwt_errors::ErrorKind::ExpiredSignature => Error::BadRequest(ClientError::new(
                "IO/JWT/EXPIRED",
                "Your JWT has expired. Please sign in again to receive and new one.",
            )),
            _ => Error::BadRequest(ClientError::new(
                "IO/JWT/EXCEPTION",
                "Something went wrong with the JWT. Please contact us for further assistance.",
            )),
        }
    }
}
