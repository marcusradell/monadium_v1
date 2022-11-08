use std::collections::HashMap;

use super::repo::types::RepoFindByEmail;
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

pub async fn handler(repo: &mut impl RepoFindByEmail, jwt: Jwt, args: &Args) -> Result<Response> {
    let identity = repo
        .find_by_email(&args.email)
        .await?
        .ok_or(Error::not_found(&args.email))?;

    password::verify(&identity.data.password_hash, &args.password)?;

    let tokens = jwt.create_tokens_from_str(HashMap::from([
        (
            "sub",
            identity.stream_id.to_hyphenated().to_string().as_str(),
        ),
        ("role", &identity.data.role),
        ("role", &identity.data.role),
        ("email", &args.email),
    ]))?;

    Ok(Response { tokens })
}
