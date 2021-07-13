use crate::io::error::Error;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub async fn handler(db: PgPool) -> Result<HttpResponse, Error> {
    println!("TODO: show identities.");

    Ok(HttpResponse::Ok().finish())
}
