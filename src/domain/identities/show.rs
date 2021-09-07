use crate::io::result::Error;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub async fn handler(_db: PgPool) -> Result<HttpResponse, Error> {
    println!("TODO: show identities.");

    Ok(HttpResponse::Ok().finish())
}
