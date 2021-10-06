use crate::io::password::verify;
use crate::io::result::Result;
use crate::{
    domain::identities::{repo::Repo, sign_in::Args},
    io::jwt::Jwt,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;

use super::handler;

pub async fn controller(
    web_repo: web::Data<Repo>,
    web_jwt: web::Data<Jwt>,
    web_args: web::Json<Args>,
) -> Result<HttpResponse> {
    let args = web_args.into_inner();
    let result = handler(
        &mut web_repo.get_ref().clone(),
        verify,
        web_jwt.get_ref().clone(),
        Utc::now(),
        &args.email,
        &args.password,
    )
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
