use serde::{Deserialize, Serialize};

use crate::io::event_store::types::Event;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct CreatedData {
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub type CreatedEvent = Event<CreatedData>;

pub const EVENT_TYPE: &str = "IDENTITIES/CREATED";
