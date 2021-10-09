mod test;
mod controller;
pub use controller::controller;

use super::{
    repo::{
        types::{RepoCreate, RepoFindByEmail},
    },
    sign_in,
    types::CreatedData,
};
use crate::io::jwt::Jwt;
use crate::io::password::{Hash, Verify};
use crate::io::result::Error;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler(
    args: Args,
    owner_email: &str,
    owner_password: &str,
    cid: Uuid,
    verify: Verify,
    hash: Hash,
    jwt: Jwt,
    repo: &mut (impl RepoCreate + RepoFindByEmail),
    now: DateTime<Utc>,
    id: Uuid,
) -> Result<sign_in::Response, Error> {
    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let exists = repo.find_by_email(&args.email).await?;

    match exists {
        // Email found, try signing them in instead of creating a new identity.
        Some(_) => {
            return sign_in::handler(repo, verify, jwt, now, &args.email, &args.password).await;
        }
        None => {
            let password_hash = hash(&args.password)?;
            let data = CreatedData {
                email: args.email.clone(),
                password_hash,
                role: role.to_string(),
            };
            repo.create(id, data, cid, now).await?;
            let result = sign_in::Response {
                jwt: jwt.encode(&id, &role, &args.email, now.timestamp())?,
            };

            Ok(result)
        }
    }
}


