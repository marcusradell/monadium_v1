use super::EventData;
use crate::io::{
    event_store::types::Event,
    result::{ClientError, Error},
};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Args {
    id: Uuid,
}

pub async fn handler(db: &PgPool, args: Args) -> Result<Event<EventData>, Error> {
    let result = sqlx::query_as::<_, Event<EventData>>(
        "select * from identities.events where event_type = IDENTITIES/CREATED and stream_id = $1",
    )
    .fetch_all(db)
    .await?;

    // We only support a single CREATED event, so no reduction is needed.
    // TODO: Make sure we have at least one item.
    let result = result
        .get(0)
        .ok_or(Error::BadRequest(ClientError::new(
            "NOT_FOUND",
            &format!("Could not find ID {}", args.id),
        )))?
        .clone();

    Ok(result)
}

pub async fn controller(
    db: web::Data<PgPool>,
    query: web::Path<Args>,
) -> Result<HttpResponse, Error> {
    let result = handler(&db.get_ref().clone(), query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}
