use super::Event;
use crate::io::{error::Error, jwt::Jwt};
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
    // TODO: Fix unwraps.
    let (_, bearer_token) = req
        .headers()
        .get("Authorization")
        .expect("Missing Authorization header")
        .to_str()
        .expect("Can't to_str.")
        .split_once("Bearer ")
        .expect("Failed to split on Bearer.");

    let decoded_token = jwt.decode(bearer_token.into())?;

    println!("{:?}", decoded_token);

    let result = handler(db.get_ref()).await?;

    Ok(HttpResponse::Ok().json(result))
}
