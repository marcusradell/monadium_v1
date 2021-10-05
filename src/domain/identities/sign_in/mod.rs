use super::repo::types::RepoFindByEmail;
use crate::io::jwt::Jwt;
use crate::io::password::types::PasswordVerifier;
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
    password: impl PasswordVerifier,
    jwt: Jwt,
    now: DateTime<Utc>,
    args: Args,
) -> Result<Response, Error> {
    let identity = repo
        .find_by_email(&args.email)
        .await?
        .ok_or(Error::BadRequest(ClientError::new(
            "NOT_FOUND",
            &format!("Could not find an identity with email {}", args.email),
        )))?;

    let verify_result = password.verify(&identity.data.password_hash, &args.password)?;

    // TODO: handle false result inside verify.
    match verify_result {
        false => Err(Error::BadRequest(ClientError::new(
            "AUTHENTICATION_FAILED",
            "Wrong email or password.",
        ))),
        true => {
            let encoded_jwt = jwt.encode(
                &identity.stream_id,
                &identity.data.role,
                &args.email.clone(),
                now.timestamp(),
            )?;
            Ok(Response { jwt: encoded_jwt })
        }
    }
}