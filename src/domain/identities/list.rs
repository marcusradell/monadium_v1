use super::Event;
use crate::io::error::Error;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub async fn handler(db: PgPool) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as::<_, Event>("select * from events")
        .fetch_all(&db)
        .await?;
    Ok(HttpResponse::Ok().json(result))
}
