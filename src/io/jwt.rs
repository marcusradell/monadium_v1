use super::result::Error;
use crate::io::result::ClientError;
use jsonwebtoken::{
    errors as jwt_errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub id: Uuid,
    pub role: String,
    pub email: String,
    pub exp: i64,
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

    #[allow(dead_code)]
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
        now: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                id: *id,
                role: role.into(),
                email: email.into(),
                exp: now + 15 * 60,
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
            jwt_errors::ErrorKind::ExpiredSignature => {
                Error::BadRequest(ClientError::auth_token_expired())
            }
            _ => Error::BadRequest(ClientError::internal_error()),
        }
    }
}
