use actix_web::{web, HttpResponse};
use dev_api::{jwt::Jwt, Result};

use crate::domain::identities::repo::Repo;

use super::{handler, Args};

pub async fn controller(
    web_repo: web::Data<Repo>,
    web_jwt: web::Data<Jwt>,
    web_args: web::Json<Args>,
) -> Result<HttpResponse> {
    let args = web_args.into_inner();
    let result = handler(
        &mut web_repo.get_ref().clone(),
        web_jwt.get_ref().clone(),
        &args,
    )
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
