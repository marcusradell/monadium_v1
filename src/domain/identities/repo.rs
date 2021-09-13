use crate::io::result::Result;
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use super::EventData;

const EVENT_TYPE: &str = "IDENTITIES/CREATED";

pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create(&self, id: Uuid, data: EventData, cid: Uuid) -> Result<()> {
        let data = Json(data);

        sqlx::query!(
            r#"
    insert into identities.events
    (stream_id, version, event_type, data, cid) VALUES
    ( $1, $2, $3, $4, $5 )
    returning sequence_num
            "#,
            id,
            1,
            EVENT_TYPE,
            data as _,
            cid
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}
