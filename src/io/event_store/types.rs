use crate::io::result::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Event<T: Clone> {
    pub sequence_num: i64,
    pub stream_id: Uuid,
    pub version: i64,
    pub event_type: String,
    pub data: Json<T>,
    pub meta: Json<EventMeta>,
    pub inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct EventMeta {
    pub cid: Uuid,
}

#[async_trait]
pub trait EventStorer<T: Clone> {
    async fn add(
        &mut self,
        event_type: &str,
        version: i64,
        stream_id: Uuid,
        data: T,
        cid: Uuid,
    ) -> Result<()>;

    async fn list(&self) -> Result<Vec<Event<T>>>;
}
