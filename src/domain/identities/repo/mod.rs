pub mod types;

use super::types::{CreatedData, CreatedEvent, EVENT_TYPE};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dev_api::Result;
use sqlx::{types::Json, PgPool};
use types::{RepoCreate, RepoFindByEmail, RepoList};
use uuid::Uuid;

#[derive(Clone)]
pub struct Repo {
    db: PgPool,
}

#[async_trait]
impl RepoCreate for Repo {
    async fn create(
        &mut self,
        id: Uuid,
        data: CreatedData,
        cid: Uuid,
        inserted_at: DateTime<Utc>,
    ) -> Result<()> {
        let data = Json(data);

        sqlx::query!(
            r#"
    insert into identities.events
    (stream_id, version, event_type, data, cid, inserted_at) VALUES
    ( $1, $2, $3, $4, $5, $6 )
    returning sequence_num
            "#,
            id,
            1,
            EVENT_TYPE,
            data as _,
            cid,
            inserted_at
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl RepoFindByEmail for Repo {
    async fn find_by_email(&mut self, email: &str) -> Result<Option<Event<CreatedData>>> {
        Ok(sqlx::query_as!(
            FindByEmailResult,
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
            data->>'email' = $1
            order by sequence_num asc"#,
            email
        )
        .fetch_optional(&self.db)
        .await?)
    }
}

#[async_trait]
impl RepoList for Repo {
    async fn list(&mut self) -> Result<Vec<Event<CreatedData>>> {
        let result = sqlx::query_as::<_, CreatedEvent>("select * from identities.events")
            .fetch_all(&self.db)
            .await?;

        // We only support a single CREATED event, so no reduction is needed.
        Ok(result)
    }
}

impl Repo {
    pub fn new(db: &PgPool) -> Self {
        Self { db: db.clone() }
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

type FindByEmailResult = Event<CreatedData>;
