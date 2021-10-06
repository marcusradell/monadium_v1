use crate::io::{
    http,
    jwt::Jwt,
    result::{ClientError, Error},
};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use super::types::CreatedEvent;

pub async fn handler(db: &PgPool) -> Result<Vec<CreatedEvent>, Error> {
    let result = sqlx::query_as::<_, CreatedEvent>("select * from identities.events")
        .fetch_all(db)
        .await?;

    // We only support a single CREATED event, so no reduction is needed.
    Ok(result)
}

pub async fn controller(
    db: web::Data<PgPool>,
    jwt: web::Data<Jwt>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let bearer_token = http::jwt_from(req)?;

    let claims = jwt.decode(bearer_token)?;

    if claims.role != "OWNER" {
        return Err(Error::BadRequest(ClientError::access_denied(
            &claims.role,
            "OWNER",
        )));
    }

    let result = handler(db.get_ref()).await?;

    Ok(HttpResponse::Ok().json(result))
}
