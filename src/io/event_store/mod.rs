use crate::{
    domain::identities::EventData,
    io::result::{Error, Result},
};
use sqlx::postgres::PgPoolOptions;

use self::types::EventStorer;

pub mod mock;
pub mod types;

#[derive(Clone)]
pub struct EventStore<T> {
    db: sqlx::PgPool,
}

impl<T: Clone> EventStore<T> {
    pub async fn new(uri: &str) -> Result<Self> {
        let db = PgPoolOptions::new().connect(uri).await?;

        Ok(Self { db })
    }

    /// Only used as an escape hatch until the API is more stable. Should be deprecated in the future.
    pub fn db(&self) -> &sqlx::PgPool {
        &self.db
    }
}

impl<T: Clone> EventStorer<T> for EventStore<T> {
    fn add(&mut self, _event: types::Event<T>) -> Result<()> {
        todo!()
    }

    fn list(&self) -> Result<Vec<types::Event<T>>> {
        todo!()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        println!("{:?}", error);
        Error::InternalServerError
    }
}

pub async fn temp() -> Result<()> {
    let es: EventStore<EventData> = EventStore::new("").await?;

    es.add(EventData {
        email: "a".into(),
        role: "b".into(),
        password_hash: "c".into(),
    });

    es.add("This should fail.");

    Ok(())
}
