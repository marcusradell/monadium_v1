use super::{EventData, EventMeta, Service};
use crate::io::error::Error;
use actix_web::HttpResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct Event {
    sequence_num: i64,
    stream_id: uuid::Uuid,
    version: i32,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    type_: String,
    data: Json<EventData>,
    meta: Json<EventMeta>,
    inserted_at: chrono::DateTime<Utc>,
}

impl Service {
    pub async fn list(self) -> Result<HttpResponse, Error> {
        let result = sqlx::query_as::<_, Event>("select * from events")
            .fetch_all(&self.db)
            .await?;

        Ok(HttpResponse::Ok().json(result))
    }
}
