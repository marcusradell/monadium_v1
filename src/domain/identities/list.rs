use super::Event;
use crate::io::{error::Error, http, jwt::Jwt};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

pub async fn handler(db: &PgPool) -> Result<Vec<Event>, Error> {
    let result = sqlx::query_as::<_, Event>("select * from events")
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn controller(
    db: web::Data<PgPool>,
    jwt: web::Data<Jwt>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let bearer_token = http::jwt_from(req)?;

    let decoded_token = jwt.decode(bearer_token)?;

    println!("{:?}", decoded_token);

    let result = handler(db.get_ref()).await?;

    Ok(HttpResponse::Ok().json(result))
}
