use std::marker::PhantomData;

use crate::{
    domain::identities::EventData,
    io::result::{Error, Result},
};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use self::types::EventStorer;

pub mod mock;
pub mod types;

#[derive(Clone)]
pub struct EventStore<T> {
    db: sqlx::PgPool,
    phantom_type: PhantomData<T>,
}

impl<T: Clone> EventStore<T> {
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

impl<T: Clone> EventStorer<T> for EventStore<T> {
    fn list(&self) -> Result<Vec<types::Event<T>>> {
        todo!()
    }

    fn add(
        &mut self,
        event_type: &str,
        version: i64,
        stream_id: uuid::Uuid,
        data: T,
        cid: uuid::Uuid,
    ) -> Result<()> {
        todo!()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{:?}", error);
        Error::InternalServerError
    }
}

// TODO: Remove.
pub async fn temp() -> Result<()> {
    let mut es: EventStore<EventData> = EventStore::new("").await?;

    es.add(
        "IT_WORKED",
        1,
        Uuid::from_u128(1),
        EventData {
            email: "a".into(),
            role: "b".into(),
            password_hash: "c".into(),
        },
        Uuid::from_u128(2),
    )?;
    Ok(())
}
