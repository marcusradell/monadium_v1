use super::repo::Repo;
use crate::io::{
    http,
    jwt::Jwt,
    result::{ClientError, Error},
};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Args {
    id: Uuid,
}

#[derive(Serialize)]
pub struct Identity {
    id: Uuid,
    version: i32,
    created_at: DateTime<Utc>,
    email: String,
    role: String,
}

pub async fn handler(repo: &Repo, args: Args) -> Result<Identity, Error> {
    let result = repo.show(&args.id).await?;

    Ok(Identity {
        id: result.stream_id,
        version: result.version,
        created_at: result.inserted_at,
        email: result.data.email.clone(),
        role: result.data.role.clone(),
    })
}

pub async fn controller(
    jwt: web::Data<Jwt>,
    req: HttpRequest,
    repo: web::Data<Repo>,
    query: web::Path<Args>,
) -> Result<HttpResponse, Error> {
    let bearer_token = http::jwt_from(req)?;

    let claims = jwt.decode(bearer_token)?;

    if claims.role != "OWNER" && claims.id != query.id {
        return Err(Error::BadRequest(ClientError::access_denied(
            &claims.role,
            "OWNER",
        )));
    }

    let result = handler(&repo, query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}
