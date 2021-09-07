use crate::io::result::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Event<T: Clone> {
    pub sequence_num: i64,
    pub stream_id: uuid::Uuid,
    pub version: i32,
    pub event_type: String,
    pub data: Json<T>,
    pub meta: Json<EventMeta>,
    pub inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct EventMeta {
    pub cid: Uuid,
}

pub trait EventStorer<T: Clone> {
    fn add(&mut self, event: Event<T>) -> Result<()>;

    fn list(&self) -> Result<Vec<Event<T>>>;
}
