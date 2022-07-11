use std::collections::HashMap;

use super::repo::types::RepoFindByEmail;
use chrono::{DateTime, Utc};
use dev_api::{
    jwt::{tokens::Tokens, Jwt},
    password, Error,
};
use serde::{Deserialize, Serialize};
mod controller;
pub use self::controller::*;
use dev_api::Result;
// mod test;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Response {
    pub tokens: Tokens,
}

pub async fn handler(
    repo: &mut impl RepoFindByEmail,
    jwt: Jwt,
    now: DateTime<Utc>,
    email: &str,
    password: &str,
) -> Result<Response> {
    let identity = repo
        .find_by_email(email)
        .await?
        .ok_or(Error::not_found(email))?;

    password::verify(&identity.data.password_hash, password)?;

    let tokens = jwt.create_tokens(HashMap::from([
        ("sub", &identity.stream_id),
        ("role", &identity.data.role),
        ("email", email),
    ]))?;

    Ok(Response { tokens })
}
