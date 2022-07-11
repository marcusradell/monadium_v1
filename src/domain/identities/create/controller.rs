use super::{handler, Args};
use crate::domain::identities::repo::Repo;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use dev_api::{
    jwt::Jwt,
    password::{hash, verify},
    Result,
};
use uuid::Uuid;

pub async fn controller<'a>(
    args: web::Json<Args>,
    jwt: web::Data<Jwt>,
    repo: web::Data<Repo>,
) -> Result<HttpResponse> {
    let owner_email =
        std::env::var("IDENTITIES_OWNER_EMAIL").expect("Missing IDENTITIES_OWNER_EMAIL.");

    let owner_password =
        std::env::var("IDENTITIES_OWNER_PASSWORD").expect("Missing IDENTITIES_OWNER_PASSWORD.");

    let args = args.into_inner();
    let now = Utc::now();
    let id = Uuid::new_v4();
    let cid = Uuid::new_v4();

    let result = handler(
        args,
        &owner_email,
        &owner_password,
        cid,
        verify,
        hash,
        jwt.get_ref().clone(),
        &mut repo.get_ref().clone(),
        now,
        id,
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
