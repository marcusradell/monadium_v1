use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

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
    pub fn new(secret: String) -> Self {
        Jwt { secret }
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
