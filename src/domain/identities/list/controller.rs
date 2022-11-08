use super::handler;
use crate::{claims::Claims, domain::identities::repo::Repo};
use actix_web::{web, HttpResponse};
use dev_api::{Authorized, Error};

pub async fn controller(repo: web::Data<Repo>, auth: Authorized) -> Result<HttpResponse, Error> {
    let claims = Claims::from_hashmap(auth.get_claims())?;

    if claims.role != "OWNER" {
        return Err(Error::access_denied(&claims.role, "OWNER"));
    }

    let result = handler(&mut repo.get_ref().clone()).await?;

    Ok(HttpResponse::Ok().json(result))
}
