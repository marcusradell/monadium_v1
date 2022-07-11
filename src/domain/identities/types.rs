use chrono::{DateTime, Utc};
use dev_api::password;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

use crate::event::Event;

pub const CREATED: &str = "IDENTITIES/CREATED";
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct CreatedData {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

impl CreatedData {
    pub fn mock_member() -> (Self, String) {
        let password = "correct_password";

        (
            Self {
                email: "existing_member@example.com".into(),
                password_hash: password::hash(password).unwrap(),
                role: "MEMBER".into(),
            },
            password.into(),
        )
    }
}

pub type CreatedEvent = Event<CreatedData>;

impl CreatedEvent {
    pub fn new(
        stream_id: Uuid,
        sequence_num: i64,
        data: CreatedData,
        cid: Uuid,
        inserted_at: DateTime<Utc>,
    ) -> Self {
        CreatedEvent {
            stream_id,
            version: 1,
            event_type: CREATED.to_string(),
            sequence_num,
            data: Json(data),
            cid,
            inserted_at,
        }
    }
}

pub const EVENT_TYPE: &str = "IDENTITIES/CREATED";
