use std::collections::HashMap;

use dev_api::{jwt::TokenType, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    exp: i64,
    r#type: TokenType,
    email: String,
    role: String,
}

fn get_value(
    hashmap: &HashMap<String, serde_json::Value>,
    name: &str,
) -> dev_api::Result<serde_json::Value> {
    match hashmap.get(name) {
        Some(value) => Ok(value.clone()),
        None => Err(Error::identity_invalid()),
    }
}

fn get_string(hashmap: &HashMap<String, serde_json::Value>, name: &str) -> dev_api::Result<String> {
    let value = get_value(hashmap, name)?;
    match value.as_str() {
        Some(val) => Ok(val.to_string()),
        None => Err(Error::identity_invalid()),
    }
}

impl Claims {
    pub fn from_hashmap(hashmap: &HashMap<String, serde_json::Value>) -> dev_api::Result<Self> {
        let sub = get_string(hashmap, "sub")?;
        let email = get_string(hashmap, "email")?;
        let exp = match get_value(hashmap, "exp")?.as_i64() {
            Some(value) => value,
            None => return Err(Error::identity_invalid()),
        };

        let r#type = get_string(hashmap, "type")?.parse()?;

        let role = get_string(hashmap, "role")?;

        Ok(Self {
            sub,
            exp,
            r#type,
            email,
            role,
        })
    }

    pub fn get_subject(&self) -> String {
        self.sub.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_role(&self) -> String {
        self.role.clone()
    }
}
