use std::collections::HashMap;

mod controller;
// mod test;

pub use controller::controller;
use dev_api::{jwt::Jwt, password, Result};

use super::{
    repo::types::{RepoCreate, RepoFindByEmail},
    sign_in,
    types::CreatedData,
};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler<'a>(
    args: Args,
    owner_email: &str,
    owner_password: &str,
    cid: Uuid,
    verify: password::Verify,
    hash: password::Hash,
    jwt: Jwt,
    repo: &mut (impl RepoCreate + RepoFindByEmail),
    now: DateTime<Utc>,
    id: Uuid,
) -> Result<sign_in::Response> {
    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let exists = repo.find_by_email(&args.email).await?;

    match exists {
        // Email found, try signing them in instead of creating a new identity.
        Some(_) => {
            return sign_in::handler(
                repo,
                jwt,
                now,
                &sign_in::Args {
                    email: args.email,
                    password: args.password,
                },
            )
            .await;
        }
        None => {
            let password_hash = hash(&args.password)?;
            let data = CreatedData {
                email: args.email,
                password_hash,
                role: role.to_string(),
            };
            let email = data.email.clone();

            repo.create(id, data, cid, now).await?;

            let result = sign_in::Response {
                tokens: jwt.create_tokens_from_str(HashMap::from([
                    ("sub", id.to_hyphenated().to_string().as_str()),
                    ("email", &email),
                ]))?,
            };

            Ok(result)
        }
    }
}
