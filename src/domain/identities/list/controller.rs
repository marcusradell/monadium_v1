use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

use crate::io::{http, jwt::Jwt, result::{ClientError, Error}};

use super::handler;


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
