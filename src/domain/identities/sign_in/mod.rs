use super::repo::types::RepoFindByEmail;
use chrono::{DateTime, Utc};
use dev_api::{
    jwt::{tokens::Tokens, Jwt},
    Error,
};
use serde::{Deserialize, Serialize};
mod controller;
pub use self::controller::*;
mod test;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Response<'a> {
    pub tokens: Tokens<'a>,
}

pub async fn handler<'a>(
    repo: &mut impl RepoFindByEmail,
    jwt: Jwt,
    now: DateTime<Utc>,
    email: &str,
    password: &str,
) -> Result<Response<'a>> {
    let identity = repo
        .find_by_email(email)
        .await?
        .ok_or(Error::not_found(email))?;

    verify(&identity.data.password_hash, password)?;

    let tokens = jwt.encode(
        &identity.stream_id,
        &identity.data.role,
        email,
        now.timestamp(),
    )?;
    Ok(Response { tokens })
}
