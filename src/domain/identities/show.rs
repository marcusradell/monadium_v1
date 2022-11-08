use crate::claims::Claims;

use super::repo::Repo;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use dev_api::{Authorized, Error, Result};
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

pub async fn handler(repo: &Repo, args: Args) -> Result<Identity> {
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
    repo: web::Data<Repo>,
    query: web::Path<Args>,
    auth: Authorized,
) -> Result<HttpResponse> {
    let claims = Claims::from_hashmap(auth.get_claims())?;

    if claims.role != "OWNER" && claims.sub != query.id.to_string() {
        return Err(Error::access_denied(&claims.role, "OWNER"));
    }

    let result = handler(&repo, query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}
