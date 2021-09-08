use self::types::EventStorer;
use crate::io::{
    event_store::types::EventMeta,
    result::{Error, Result},
};
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, types::Json};
use std::marker::PhantomData;
use uuid::Uuid;

pub mod mock;
pub mod types;

#[derive(Clone)]
pub struct EventStore<T: Clone + sqlx::Type<sqlx::Postgres>> {
    db: sqlx::PgPool,
    phantom_type: PhantomData<T>,
}

impl<T: Clone + sqlx::Type<sqlx::Postgres>> EventStore<T> {
    pub async fn new(uri: &str) -> Result<Self> {
        let db = PgPoolOptions::new().connect(uri).await?;

        Ok(Self {
            db,
            phantom_type: PhantomData,
        })
    }

    /// Only used as an escape hatch until the API is more stable. Should be deprecated in the future.
    pub fn db(&self) -> &sqlx::PgPool {
        &self.db
    }
}

#[async_trait]
impl<T: Clone + sqlx::Type<sqlx::Postgres>> EventStorer<T> for EventStore<T> {
    async fn list(&self) -> Result<Vec<types::Event<T>>> {
        todo!()
    }

    async fn add(
        &mut self,
        event_type: &str,
        version: i64,
        stream_id: uuid::Uuid,
        data: T,
        cid: uuid::Uuid,
    ) -> Result<()> {
        let meta = Json(EventMeta {
            cid: Uuid::new_v4(),
        });

        let id = Uuid::new_v4();

        sqlx::query!(
            r#"
    insert into events
    (stream_id, version, event_type, data, meta) VALUES
    ( $1, $2, $3, $4, $5 )
    returning sequence_num
            "#,
            id,
            1,
            event_type,
            data as _,
            meta as _
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{:?}", error);
        Error::InternalServerError
    }
}
