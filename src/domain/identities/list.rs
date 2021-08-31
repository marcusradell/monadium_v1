use super::Event;
use crate::io::error::Error;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn handler(db: &PgPool) -> Result<Vec<Event>, Error> {
    let result = sqlx::query_as::<_, Event>("select * from events")
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn controller(db: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let result = handler(db.get_ref()).await?;

    Ok(HttpResponse::Ok().json(result))
}
