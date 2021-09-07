use super::result::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

pub mod mock;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Event<T: Clone> {
    sequence_num: i64,
    stream_id: uuid::Uuid,
    version: i32,
    event_type: String,
    data: Json<T>,
    meta: Json<EventMeta>,
    inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct EventMeta {
    pub cid: Uuid,
}

pub trait EventStorer<T: Clone> {
    fn add(&mut self, event: Event<T>) -> Result<()>;

    fn list(&self) -> Result<Vec<Event<T>>>;
}
