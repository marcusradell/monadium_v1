use super::{
    repo::{types::RepoCreate, Repo},
    sign_in,
    types::CreatedData,
};
use crate::io::jwt::Jwt;
use crate::io::password;
use crate::io::result::Error;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct Args {
    pub email: String,
    pub password: String,
}

pub async fn handler(
    args: Args,
    owner_email: String,
    owner_password: String,
    jwt: Jwt,
    repo: &mut Repo,
) -> Result<sign_in::Response, Error> {
    let role = if owner_email == args.email && owner_password == args.password {
        "OWNER"
    } else {
        "MEMBER"
    };

    let exists = repo.exists_by_email(&args.email).await?;

    match exists {
        // Email found, try signing them in instead of creating a new identity.
        Some(_) => {
            return sign_in::handler(
                repo,
                jwt,
                Utc::now().timestamp(),
                sign_in::Args {
                    email: args.email.clone(),
                    password: args.password,
                },
            )
            .await;
        }
        None => {
            let password_hash = password::hash(&args.password)?;
            let data = CreatedData {
                email: args.email.clone(),
                password_hash,
                role: role.to_string(),
            };
            let cid = Uuid::new_v4();
            let id = Uuid::new_v4();
            repo.create(id, data, cid).await?;
            let result = sign_in::Response {
                jwt: jwt.encode(&id, &role, &args.email, Utc::now().timestamp())?,
            };

            Ok(result)
        }
    }
}

pub async fn controller(
    args: web::Json<Args>,
    jwt: web::Data<Jwt>,
    repo: web::Data<Repo>,
) -> Result<HttpResponse, Error> {
    let owner_email =
        std::env::var("IDENTITIES_OWNER_EMAIL").expect("Missing IDENTITIES_OWNER_EMAIL.");

    let owner_password =
        std::env::var("IDENTITIES_OWNER_PASSWORD").expect("Missing IDENTITIES_OWNER_PASSWORD.");

    let args = args.into_inner();

    let result = handler(
        args,
        owner_email,
        owner_password,
        jwt.get_ref().clone(),
        &mut repo.get_ref().clone(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
