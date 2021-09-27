use crate::io::{
    event_store::types::Event,
    result::{Error, Result},
};
use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use super::CreatedData;

const EVENT_TYPE: &str = "IDENTITIES/CREATED";

type CreatedEvent = Event<CreatedData>;

#[derive(Clone)]
pub struct Repo {
    db: PgPool,
}

impl Repo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create(&self, id: Uuid, data: CreatedData, cid: Uuid) -> Result<()> {
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

    pub async fn exists_by_email(&self, email: &str) -> Result<Option<()>> {
        let result = sqlx::query!(
            r#"select * from identities.events where event_type = $1 and data->>'email' = $2 limit 1"#,
            EVENT_TYPE,
            email.clone()
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(result.and(Some(())))
    }

    pub async fn show(&self, id: &Uuid) -> Result<Event<CreatedData>> {
        sqlx::query_as!(
            CreatedEvent,
            r#"select
            stream_id,
            sequence_num,
            version,
            event_type,
            cid,
            inserted_at,
            data as "data: Json<CreatedData>"
            from identities.events
            where
            event_type = $1 and
            stream_id = $2
            limit 1"#,
            EVENT_TYPE,
            id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(Error::InternalServerError)
    }
}
