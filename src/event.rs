use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Event<T: Clone> {
    pub sequence_num: i64,
    pub stream_id: Uuid,
    pub version: i32,
    pub event_type: String,
    pub data: Json<T>,
    pub cid: Uuid,
    pub inserted_at: DateTime<Utc>,
}
