use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

use crate::io::event_store::types::Event;

pub const CREATED: &str = "IDENTITIES/CREATED";
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct CreatedData {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub type CreatedEvent = Event<CreatedData>;

impl CreatedEvent {
    pub fn new(stream_id: Uuid, sequence_num: i64, data: CreatedData, cid: Uuid) -> Self {
        CreatedEvent {
            stream_id,
            version: 1,
            event_type: CREATED.to_string(),
            sequence_num,
            data: Json(data),
            cid,
            inserted_at: Utc::now(),
        }
    }
}

pub const EVENT_TYPE: &str = "IDENTITIES/CREATED";
