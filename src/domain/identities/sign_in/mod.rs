use super::repo::types::RepoFindByEmail;
use crate::io::jwt::Jwt;
use crate::io::password::Verify;
use crate::io::result::{ClientError, Error};
use chrono::{DateTime, Utc};
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
pub struct Response {
    pub jwt: String,
}

pub async fn handler(
    repo: &mut impl RepoFindByEmail,
    verify: Verify,
    jwt: Jwt,
    now: DateTime<Utc>,
    email: &str,
    password: &str,
) -> Result<Response, Error> {
    let identity = repo
        .find_by_email(email)
        .await?
        .ok_or(Error::BadRequest(ClientError::not_found(email)))?;

    let verify_result = verify(&identity.data.password_hash, password)?;

    // TODO: handle false result inside verify.
    match verify_result {
        false => Err(Error::BadRequest(ClientError::auth_failed())),
        true => {
            let encoded_jwt = jwt.encode(
                &identity.stream_id,
                &identity.data.role,
                email,
                now.timestamp(),
            )?;
            Ok(Response { jwt: encoded_jwt })
        }
    }
}
